# agentfsd

`agentfsd` is the deployable AgentFS gateway/daemon track.

Primary goals:

- simple single-binary deployment;
- remote AgentFS service for VPS/server environments;
- low-dependency container images;
- protocol-conformant HTTP/service surface;
- reusable backend connections without embedding credentials in requests.

Go is the recommended initial daemon language because deployment simplicity is the dominant concern here. The daemon must still consume AgentFS protocol semantics rather than developing a Go-specific filesystem contract.

Status: foundation placeholder. No daemon implementation is claimed yet.