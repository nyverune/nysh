use std::process::Command;
use strum_macros::EnumString;

use super::utils::create_dir;

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

pub struct GenerateKeyProps {
    alias: String,
    email: String,
    algorithm: Algorithm,
    passphrasse: String,
    bits: u32,
}

pub fn generate_key(props: GenerateKeyProps) -> std::io::Result<()> {
    let ssh_dir = home::home_dir().unwrap().join(".ssh");

    create_dir(&ssh_dir)?;

    let key_path = ssh_dir.join(&props.alias);
    let key_pub_path = ssh_dir.join(format!("{}.pub", &props.alias));

    if key_path.exists() || key_pub_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Alias {} already exists.", props.alias),
        ));
    }

    let status = Command::new("ssh-keygen")
        .args([
            "-t",
            &props.algorithm.to_string(),
            "-C",
            &props.email,
            "-N",
            &props.passphrasse,
            "-f",
            key_path.to_str().unwrap(),
            "-q",
        ])
        .status()
        .expect("ssh-keygen failed");

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ssh-keygen exited with {}", status),
        ));
    }

    Ok(())
}
