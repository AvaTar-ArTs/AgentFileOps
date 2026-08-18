use agent_file_ops_ssh::{AgentFileOpsSshSession, SshTransportConfig, TransportError};
use std::fs;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

struct OpenSshFixture {
    _dir: TempDir,
    root: PathBuf,
    username: String,
    port: u16,
    client_key: PathBuf,
    known_hosts: PathBuf,
    child: Child,
}

impl OpenSshFixture {
    fn start() -> io::Result<Self> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("remote-root");
        fs::create_dir(&root)?;
        fs::write(root.join("hello.txt"), b"hello from ssh")?;
        std::os::unix::fs::symlink("hello.txt", root.join("hello-link"))?;

        let host_key = dir.path().join("host_key");
        let client_key = dir.path().join("client_key");
        run_keygen(&host_key)?;
        run_keygen(&client_key)?;
        let authorized_keys = dir.path().join("authorized_keys");
        fs::copy(client_key.with_extension("pub"), &authorized_keys)?;

        let username = std::env::var("USER").unwrap_or_else(|_| "root".to_string());
        let port = free_port()?;
        let config = dir.path().join("sshd_config");
        fs::write(
            &config,
            format!(
                "Port {port}\nListenAddress 127.0.0.1\nHostKey {}\nPidFile {}\nAuthorizedKeysFile {}\nPasswordAuthentication no\nKbdInteractiveAuthentication no\nUsePAM no\nPermitRootLogin yes\nStrictModes no\nAllowUsers {username}\nSubsystem sftp internal-sftp\nLogLevel ERROR\n",
                host_key.display(),
                dir.path().join("sshd.pid").display(),
                authorized_keys.display(),
            ),
        )?;
        let known_hosts = dir.path().join("known_hosts");
        write_known_host(&known_hosts, port, &host_key)?;

        let child = Command::new("/usr/sbin/sshd")
            .args(["-D", "-e", "-f"])
            .arg(&config)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        wait_for_port(port)?;

        Ok(Self {
            _dir: dir,
            root,
            username,
            port,
            client_key,
            known_hosts,
            child,
        })
    }

    fn config(&self, credential: &Path, known_hosts: &Path) -> SshTransportConfig {
        SshTransportConfig::new(
            "127.0.0.1",
            self.port,
            known_hosts.to_string_lossy(),
            credential.to_string_lossy(),
        )
        .with_username(&self.username)
        .with_inline_read_limit(64)
    }
}

impl Drop for OpenSshFixture {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn run_keygen(path: &Path) -> io::Result<()> {
    let mut command = Command::new("/usr/bin/ssh-keygen");
    command.args(["-q", "-t", "ed25519", "-N", "", "-f"]);
    command.arg(path);
    let output = command.output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(String::from_utf8_lossy(&output.stderr)))
    }
}

fn free_port() -> io::Result<u16> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    Ok(listener.local_addr()?.port())
}

fn wait_for_port(port: u16) -> io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "sshd did not start before timeout",
    ))
}

fn public_key_fields(path: &Path) -> io::Result<(String, String)> {
    let content = fs::read_to_string(path.with_extension("pub"))?;
    let mut fields = content.split_whitespace();
    let key_type = fields
        .next()
        .ok_or_else(|| io::Error::other("public key has no type"))?;
    let key_data = fields
        .next()
        .ok_or_else(|| io::Error::other("public key has no data"))?;
    Ok((key_type.to_string(), key_data.to_string()))
}

fn write_known_host(path: &Path, port: u16, host_key: &Path) -> io::Result<()> {
    let (key_type, key_data) = public_key_fields(host_key)?;
    fs::write(path, format!("[127.0.0.1]:{port} {key_type} {key_data}\n"))
}

fn fixture() -> OpenSshFixture {
    OpenSshFixture::start().expect("local OpenSSH fixture must start")
}

#[tokio::test(flavor = "multi_thread")]
async fn unknown_host_key_is_rejected() {
    let fixture = fixture();
    let empty_known_hosts = fixture._dir.path().join("empty_known_hosts");
    fs::write(&empty_known_hosts, "").unwrap();
    let mut session =
        AgentFileOpsSshSession::new(fixture.config(&fixture.client_key, &empty_known_hosts));
    assert!(matches!(
        session.connect().await,
        Err(TransportError::UnknownHostKey { .. })
    ));
}

#[tokio::test(flavor = "multi_thread")]
async fn wrong_host_key_is_rejected() {
    let fixture = fixture();
    let wrong_host_key = fixture._dir.path().join("wrong_host_key");
    run_keygen(&wrong_host_key).unwrap();
    let wrong_known_hosts = fixture._dir.path().join("wrong_known_hosts");
    write_known_host(&wrong_known_hosts, fixture.port, &wrong_host_key).unwrap();
    let mut session =
        AgentFileOpsSshSession::new(fixture.config(&fixture.client_key, &wrong_known_hosts));
    assert!(matches!(
        session.connect().await,
        Err(TransportError::HostKeyMismatch { .. })
    ));
}

#[tokio::test(flavor = "multi_thread")]
async fn bad_credential_is_rejected() {
    let fixture = fixture();
    let wrong_key = fixture._dir.path().join("wrong_client_key");
    run_keygen(&wrong_key).unwrap();
    let mut session = AgentFileOpsSshSession::new(fixture.config(&wrong_key, &fixture.known_hosts));
    assert!(matches!(
        session.connect().await,
        Err(TransportError::AuthenticationFailed)
    ));
}

#[tokio::test(flavor = "multi_thread")]
async fn authenticated_sftp_operations_are_bounded_and_additive() {
    let fixture = fixture();
    let mut session =
        AgentFileOpsSshSession::new(fixture.config(&fixture.client_key, &fixture.known_hosts));
    session.connect().await.unwrap();
    {
        let sftp = session.open_sftp().await.unwrap();
        let entries = sftp
            .list(fixture.root.to_str().unwrap(), Some(10))
            .await
            .unwrap();
        assert!(entries.iter().any(|entry| entry.name == "hello.txt"));

        let stat = sftp
            .stat(fixture.root.join("hello.txt").to_str().unwrap())
            .await
            .unwrap();
        assert_eq!(stat.file_type, agent_file_ops_ssh::RemoteFileType::File);
        let lstat = sftp
            .lstat(fixture.root.join("hello-link").to_str().unwrap())
            .await
            .unwrap();
        assert_eq!(lstat.file_type, agent_file_ops_ssh::RemoteFileType::Symlink);

        let data = sftp
            .read(fixture.root.join("hello.txt").to_str().unwrap(), 0, 64)
            .await
            .unwrap();
        assert_eq!(data, b"hello from ssh");
        assert!(matches!(
            sftp.read(fixture.root.join("hello.txt").to_str().unwrap(), 0, 4)
                .await,
            Err(TransportError::ReadLimitExceeded { limit: 4 })
        ));

        let new_path = fixture.root.join("created.txt");
        let result = sftp
            .write_new(new_path.to_str().unwrap(), b"created")
            .await
            .unwrap();
        assert_eq!(result.bytes_written, 7);
        assert!(matches!(
            sftp.write_new(new_path.to_str().unwrap(), b"again").await,
            Err(TransportError::Conflict(_))
        ));
    }
    session.disconnect().await.unwrap();
}
