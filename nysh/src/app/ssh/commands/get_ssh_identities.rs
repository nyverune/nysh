use super::types::{Algorithm, AuthStatus, SshIdentity};
use crate::config::AppConfig;

pub fn get_ssh_identities(_config: &AppConfig) -> Vec<SshIdentity> {
    vec![
        SshIdentity {
            alias: "github-personal".to_string(),
            key_type: Algorithm::Ed25519,
            key_path: "~/.ssh/id_github_personal".to_string(),
            public_key: "ssh-ed25519 AAAAC3Nza...4fG7 octocat@me".to_string(),
            fingerprint: "SHA256:9x2k...Q7p".to_string(),
            auth_status: AuthStatus::Authenticated("octocat".to_string()),
        },
        SshIdentity {
            alias: "github-work".to_string(),
            key_type: Algorithm::Ed25519,
            key_path: "~/.ssh/id_github_work".to_string(),
            public_key: "ssh-ed25519 AAAAC3Nza...8hJ2 company-bot@work".to_string(),
            fingerprint: "SHA256:4a1b...Xk9".to_string(),
            auth_status: AuthStatus::NotTested,
        },
        SshIdentity {
            alias: "github-university".to_string(),
            key_type: Algorithm::Ecdsa,
            key_path: "~/.ssh/id_rsa_uni".to_string(),
            public_key: "ssh-rsa AAAAB3Nza...1aB2 student@uni".to_string(),
            fingerprint: "SHA256:7c3d...M2p".to_string(),
            auth_status: AuthStatus::Failed("Permission denied".to_string()),
        },
    ]
}
