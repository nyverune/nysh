use super::types::{Algorithm, AuthStatus, SshIdentity};
use crate::config::AppConfig;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return format!("{}/{}", home, &path[2..]);
        }
    }
    path.to_string()
}

fn get_public_key(path: &str) -> (String, Algorithm) {
    let expanded = expand_tilde(path);
    let pub_path = format!("{}.pub", expanded);

    if let Ok(content) = fs::read_to_string(Path::new(&pub_path)) {
        let algorithm = content
            .split_whitespace()
            .next()
            .map(Algorithm::parse)
            .unwrap_or(Algorithm::Unknown);

        return (content.trim().to_string(), algorithm);
    }

    (String::new(), Algorithm::Unknown)
}

fn get_fingerprint(path: &str) -> io::Result<String> {
    let expanded = expand_tilde(path);
    let pub_path = format!("{}.pub", expanded);

    let output = Command::new("ssh-keygen")
        .args(["-lf", &pub_path])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("ssh-keygen failed: {}", stderr.trim()),
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let fingerprint = stdout
        .split_whitespace()
        .nth(1)
        .unwrap_or("failed")
        .to_string();

    Ok(fingerprint)
}

fn build_ssh_identity(host: &str, host_name: &str, identity_file: &str) -> SshIdentity {
    let (public_key, key_type) = get_public_key(identity_file);
    let fingerprint = match get_fingerprint(identity_file) {
        Ok(fp) => fp,
        Err(e) => format!("error: {}", e), // Así lo verás en la TUI
    };

    SshIdentity {
        alias: host.to_string(),
        key_type: key_type,
        key_path: identity_file.to_string(),
        host_name: host_name.to_string(),
        public_key: public_key.to_string(),
        fingerprint,
        auth_status: AuthStatus::NotTested,
    }
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
                entries.push(build_ssh_identity(&host, &host_name, &identity_file));
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
        entries.push(build_ssh_identity(&host, &host_name, &identity_file));
    }

    entries
}

pub fn get_ssh_identities(config: &AppConfig) -> Vec<SshIdentity> {
    let content =
        fs::read_to_string(Path::new(&config.ssh_config)).unwrap_or_else(|_| String::new());

    let entries = parse_identities(content);

    entries
}
