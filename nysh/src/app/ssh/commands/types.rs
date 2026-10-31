use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString, strum::Display)]
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
