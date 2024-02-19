//dos
//------------------------------> 2 colonne funziona

// @echo -----------------------------------------------------------------------------
// @echo si trova in questa c:\\CASA\\LINGUAGGI\\RUST\\RUST_PURO\\RUST_PURO_COMMAND_DOS\\cmd_task_kill\\
// @echo file : 



use std::process::{Command, exit};

fn main() {
    // Stampare l'intestazione
    println!("Servizi attivi su Windows 10:");
    println!("------------------------------------------");
    println!("   file SERVIZI_ATTIV_PID+NOME_(Tutti).BAT");
    println!("------------------------------------------");

    // Eseguire il comando tasklist
    let output = Command::new("tasklist")
        .arg("/fi")
        .arg("sessionname eq console")
        .output()
        .expect("Failed to execute command");

    // Verificare se il comando è stato eseguito correttamente
    if output.status.success() {
        // Converte il risultato in una stringa UTF-8
        let result = String::from_utf8_lossy(&output.stdout);

        // Stampa l'elenco dei servizi attivi
        println!("{}", result);

        // Stampa la riga finale
        println!("------------------------------------------");
        println!("L'elenco dei servizi attivi è stato salvato nel file \"%outputFile%\".");
    } else {
        // Stampa un messaggio di errore se il comando non ha avuto successo
        eprintln!("Errore durante l'esecuzione del comando: {:?}", output.status);
        exit(1);
    }
}



/*
    COMANDI PER TROVARE ED ESEGUIRE RUST
        where rustup    = dove si trova rustup  quindi trova dove è l'eseguibile

*/