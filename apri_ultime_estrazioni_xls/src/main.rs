/*
    SHELL
        per aprire la shell nel progetto digitare
            ctrl+shift+p
            open default shell in panel

    RELEASE
        cargo build --release

    PROGETTO RUST
    per aprire il progetto mdb ULTIME ESTRAZIONI
    utilizzo il dos. I parametri da cambiare:
        
        path:
            c:\\CASA\\LTT\\COLL\\XLS\\
           

        file
            ULTIME_ESTRAZIONI.xls

*/

use std::process::Command;



fn main() {
    // Parametri per la path e il file MDB
    let path = "c:\\CASA\\LTT\\COLL\\XLS\\";
    let file_name = "ULTIME_ESTRAZIONI.xls";

    // Componi il percorso completo al file MDB
    let db_path = format!("{}{}", path, file_name);

    // Eseguire il comando DOS per aprire il file MDB
    let output = Command::new("cmd")
        .args(&["/C", "start", &db_path])
        .output()
        .expect("Errore nell'esecuzione del comando.");

    // Controlla se ci sono stati errori nell'esecuzione del comando
    if output.status.success() {
        println!("Il file MDB è stato aperto con successo.");
    } else {
        eprintln!("Si è verificato un errore nell'apertura del file MDB.");
    }
}
