use super::types::{Algorithm, AuthStatus, SshIdentity};
use crate::config::AppConfig;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn parse_identities(content: String) -> Vec<SshIdentity> {
    let mut entries = Vec::new();
    let mut host: String = "".to_string();
    let mut host_name: String = "".to_string();
    let mut identity_file: String = "".to_string();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if line.to_lowercase().starts_with("host ") {
            if host != "".to_string() {
                entries.push(SshIdentity {
                    alias: host,
                    key_type: Algorithm::Ed25519,
                    key_path: identity_file,
                    host_name,
                    public_key: "".to_string(),
                    fingerprint: "".to_string(),
                    auth_status: AuthStatus::NotTested,
                });
            }

            host = line[5..].to_string();
            host_name = "".to_string();
            identity_file = "".to_string();
        } else if line.to_lowercase().starts_with("hostname ") {
            host_name = line[8..].to_string();
        } else if line.to_lowercase().starts_with("identityfile ") {
            identity_file = line[13..].to_string();
        }
    }

    if host != "".to_string() {
        entries.push(SshIdentity {
            alias: host,
            key_type: Algorithm::Ed25519,
            key_path: identity_file,
            host_name,
            public_key: "".to_string(),
            fingerprint: "".to_string(),
            auth_status: AuthStatus::NotTested,
        });
    }

    entries
}

pub fn get_ssh_identities(config: &AppConfig) -> Vec<SshIdentity> {
    let content =
        fs::read_to_string(Path::new(&config.ssh_config)).unwrap_or_else(|_| String::new());

    let entries = parse_identities(content);

    entries
}
