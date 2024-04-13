use std::process::Command;

fn main() {
    let pid_to_kill = 1116; // Sostituire con il PID effettivo del processo che si desidera terminare

    // Costruisci il comando per inviare un segnale di kill al processo specificato
    let output = Command::new("taskkill")
        .args(&["/F", "/PID", &pid_to_kill.to_string()])
        .output();

    // Verifica se il comando è stato eseguito con successo
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Il processo con PID {} è stato terminato con successo.", pid_to_kill);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Errore durante la terminazione del processo: {}", stderr);
            }
        }
        Err(err) => {
            println!("Errore durante l'esecuzione del comando: {:?}", err);
        }
    }
}
