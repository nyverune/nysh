use std::process::Command;

pub fn is_program_installed(program: &str) -> bool {
    let (cmd, arg) = if cfg!(target_os = "windows") {
        ("where", program)
    } else {
        ("which", program)
    };

    // 1. Pasa el programa y el argumento por separado
    // 2. Usa .status() para esperar a que el proceso termine
    match Command::new(cmd).arg(arg).status() {
        Ok(status) => status.success(), // Devuelve true si el comando tuvo éxito (exit code 0)
        Err(_) => false,                // El comando en sí falló al ejecutarse
    }
}

