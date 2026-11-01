use super::types::{Algorithm, AuthStatus, SshIdentity};
use crate::config::AppConfig;

// SshIdentity {
//      alias: "github-personal".to_string(),
//      key_type: Algorithm::Ed25519,
//      key_path: "~/.ssh/id_github_personal".to_string(),
//      public_key: "ssh-ed25519 AAAAC3Nza...4fG7 octocat@me".to_string(),
//      fingerprint: "SHA256:9x2k...Q7p".to_string(),
//      auth_status: AuthStatus::Authenticated("octocat".to_string()),
// }

pub fn get_ssh_identities(_config: &AppConfig) -> Vec<SshIdentity> {
    vec![]
}
