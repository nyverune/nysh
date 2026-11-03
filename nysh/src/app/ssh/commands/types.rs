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
