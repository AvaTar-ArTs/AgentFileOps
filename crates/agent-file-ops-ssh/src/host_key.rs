use crate::TransportError;
use std::path::Path;

/// Strict SSH host key verification.
///
/// Implements fail-closed host key validation:
/// - Unknown hosts are rejected
/// - Mismatched keys are rejected
/// - Only allows explicitly trusted keys
pub struct StrictHostKeyVerifier;

impl StrictHostKeyVerifier {
    /// Verify a host key against known_hosts.
    ///
    /// # Arguments
    /// * `host` - Hostname
    /// * `port` - SSH port
    /// * `key_type` - Key type identifier (e.g., "ssh-rsa", "ssh-ed25519")
    /// * `key_data` - Base64-encoded public key
    /// * `known_hosts_path` - Path to known_hosts file
    ///
    /// # Returns
    /// - `Ok(())` if key matches a line in known_hosts
    /// - `Err(UnknownHostKey)` if host is not in known_hosts
    /// - `Err(HostKeyMismatch)` if key differs from known_hosts entry
    pub fn verify(
        host: &str,
        port: u16,
        key_type: &str,
        key_data: &str,
        known_hosts_path: impl AsRef<Path>,
    ) -> Result<(), TransportError> {
        let path = known_hosts_path.as_ref();

        // Read known_hosts file
        let content = std::fs::read_to_string(path).map_err(|e| {
            TransportError::KnownHostsUnavailable(format!("failed to read known_hosts: {}", e))
        })?;

        // Parse known_hosts and search for matching entry
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse known_hosts line: [host]:port key_type key_data
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue; // Invalid line, skip
            }

            let hosts_spec = parts[0];
            let stored_key_type = parts[1];
            let stored_key_data = parts[2];

            // Parse host specification (may include port)
            if Self::matches_host_spec(hosts_spec, host, port) {
                // Found matching host entry; verify key matches
                if stored_key_type == key_type && stored_key_data == key_data {
                    return Ok(()); // ✓ Key matches
                } else {
                    return Err(TransportError::HostKeyMismatch {
                        host: host.to_string(),
                        port,
                        line: line_num + 1,
                    });
                }
            }
        }

        // Host not found in known_hosts
        Err(TransportError::UnknownHostKey {
            host: host.to_string(),
            port,
        })
    }

    /// Check if a known_hosts host specification matches the target host:port.
    fn matches_host_spec(spec: &str, target_host: &str, target_port: u16) -> bool {
        // Handle bracketed format: [host]:port
        if let Some(bracketed) = spec.strip_prefix('[') {
            if let Some(colon_idx) = bracketed.rfind(']') {
                let host_part = &bracketed[..colon_idx];
                if let Some(port_part) = bracketed[colon_idx + 1..].strip_prefix(':') {
                    if let Ok(port) = port_part.parse::<u16>() {
                        return host_part == target_host && port == target_port;
                    }
                }
            }
        }

        // Handle plain format: host (default port 22)
        if spec == target_host && target_port == 22 {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn matches_bracketed_host_port() {
        assert!(StrictHostKeyVerifier::matches_host_spec(
            "[example.com]:2222",
            "example.com",
            2222
        ));
    }

    #[test]
    fn matches_plain_host_default_port() {
        assert!(StrictHostKeyVerifier::matches_host_spec(
            "example.com",
            "example.com",
            22
        ));
    }

    #[test]
    fn rejects_mismatched_port() {
        assert!(!StrictHostKeyVerifier::matches_host_spec(
            "[example.com]:2222",
            "example.com",
            22
        ));
    }

    #[test]
    fn unknown_host_fails() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "[other.com]:22 ssh-ed25519 AAAA...").unwrap();
        temp.flush().unwrap();

        let result =
            StrictHostKeyVerifier::verify("example.com", 22, "ssh-ed25519", "AAAA...", temp.path());

        assert!(matches!(result, Err(TransportError::UnknownHostKey { .. })));
    }
}
