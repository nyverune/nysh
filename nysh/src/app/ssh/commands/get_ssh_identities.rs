use super::types::{Algorithm, AuthStatus, SshIdentity};
use crate::config::AppConfig;
use std::env;
use std::fs;
use std::path::Path;

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return format!("{}/{}", home, &path[2..]);
        }
    }
    path.to_string()
}

fn get_public_key(path: String) -> String {
    let expanded = expand_tilde(&path);
    let pub_path = format!("{}.pub", expanded);

    if let Ok(content) = fs::read_to_string(Path::new(&pub_path)) {
        return content.trim().to_string();
    }
    String::new()
}

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
                    key_path: identity_file.clone(),
                    host_name,
                    public_key: get_public_key(identity_file.clone()),
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
            identity_file = line[13..].trim_matches('"').to_string();
        }
    }

    if host != "".to_string() {
        entries.push(SshIdentity {
            alias: host,
            key_type: Algorithm::Ed25519,
            key_path: identity_file.clone(),
            host_name,
            public_key: get_public_key(identity_file.clone()),
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
