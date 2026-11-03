use std::fmt;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone, EnumString, strum::Display)]
pub enum Algorithm {
    #[strum(serialize = "ecdsa")]
    Ecdsa,

    #[strum(serialize = "ecdsa-sk")]
    EcdsaSk,

    #[strum(serialize = "ed25519")]
    Ed25519,

    #[strum(serialize = "ed25519-sk")]
    Ed25519Sk,

    #[strum(serialize = "mldsa44-ed25519")]
    Mldsa44Ed25519,

    #[strum(serialize = "rsa")]
    Rsa,

    #[strum(serialize = "unknown")]
    Unknown,
}

impl Algorithm {
    pub fn parse(s: &str) -> Algorithm {
        match s {
            // --- ed25519 ---
            "ssh-ed25519" => Algorithm::Ed25519,
            "sk-ssh-ed25519@openssh.com" => Algorithm::Ed25519Sk,

            // --- RSA ---
            // Only "ssh-rsa" carries the rsa-sha2-256/512 capabilities; the
            // rsa-sha2-* strings can also appear as key types in `.pub` files
            // on newer OpenSSH, so accept them too.
            "ssh-rsa" | "rsa-sha2-256" | "rsa-sha2-512" => Algorithm::Rsa,

            // --- ECDSA (NIST curves) ---
            "ecdsa-sha2-nistp256" | "ecdsa-sha2-nistp384" | "ecdsa-sha2-nistp521" => {
                Algorithm::Ecdsa
            }

            // --- ECDSA security key (FIDO/U2F) ---
            "sk-ecdsa-sha2-nistp256@openssh.com" => Algorithm::EcdsaSk,

            // --- Post-quantum hybrid ---
            // Matches the hybrid algorithm you already have a variant for.
            // (Exact string may vary by OpenSSH version; adjust as needed.)
            "mldsa44-ed25519" | "mldsa44-ed25519@openssh.com" => Algorithm::Mldsa44Ed25519,

            // --- Everything else: DSA, cert types, unknown ---
            // "ssh-dss", "*-cert-v01@openssh.com", "" (empty first token),
            // and anything unrecognized.
            _ => Algorithm::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SshIdentity {
    pub alias: String,
    pub key_type: Algorithm,
    pub key_path: String,
    pub host_name: String,
    pub public_key: String,
    pub fingerprint: String,
    pub auth_status: AuthStatus,
}

#[derive(Debug, Clone)]
pub enum AuthStatus {
    NotTested,
    Authenticated(String), // Save github username
    Failed(String),
}

impl fmt::Display for AuthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthStatus::NotTested => write!(f, "not tested"),
            AuthStatus::Authenticated(user) => write!(f, "authenticated ({})", user),
            AuthStatus::Failed(reason) => write!(f, "auth failed ({})", reason),
        }
    }
}
