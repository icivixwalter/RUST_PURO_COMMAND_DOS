/*
    PROGETTO RUST
    per aprire il progetto mdb SCHED_MDB
        @apro@dos_(singolo file con il @comando @bat)

     SHELL
        @shell@panel
        Terminus:open default shell in panel
        per aprire la shell nel progetto digitare
        
            ctrl+shift+p
            open default shell in panel



    utilizzo il dos. I parametri da cambiare:
        
        path:
            C:\\CASA\\LTT\\SCHED\\SCHED_MDB\\

        file
            SCHED.mdb

            Terminus:open default 

*/

use std::process::Command;



fn main() {
    // Parametri per la path e il file MDB
    
    let path = "c:\\CASA\\LINGUAGGI\\ACCESS\\PROGETTI_MDB\\MSYS_OGGETTI\\MSYS\\OBJECT\\HELP\\";
    let file_name = "APRI_HELP_SottoFormMaster_{LibreOfficeWriter}.bat";

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
