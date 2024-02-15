

/*

    RUST APRO TUTTI I SORGENTI DEI PROGETTI @MDB@ACCESS
        questo progetto crea un eseguibile .exe che utilizzando il comando
        dos o utilizzando lo standard flusso fs.
        Posso utilizzarlo per 1 o piu programmi eseguibili (.exe), in quanto il codice
        si basa su un ciclo for che esegue i parametri salvati in 
        un vettore che contiene per ogni progetto access 2 parametri
            1) path
            2) file.mdb
        Per ora a condizione che tutti programmi .exe si trovano sulla stessa
        path (per cui devi costruire diversi eseguibili e metterli sulla 
        stessa path e possono chiamare anche programmi dislocati su
        vari indirizzi.)
        In questo modo evitiamo di fare programmi con path diverse con 
        i problemi per la ricerca. Ogni progetto puo avere un solo eseguibile
        e puo essere chiamato con questo sistema ricorsivo.

        per funzionare anche su un progetto impostare i parametri del
        vettore.
        Per ora viene applicato su 1 parametro solo:
         c:\CASA\GE_CASA\GE_MARINO\GESTIONE_SPESE\
         GE_CASA_SPESE_TUTTE.mdb

        @APRO@FILE@ESEGUIBILI@.EXE_(apro file eseguibili .exe con il @dos)

        attenzione per il terminale usa IL DOS e non

        c'è bisogno delle dipendenze nel toml.

        Basta utilizzare il vettore per aprire file eseguibili.exe

       

        @shell@panel
        Terminus:open default shell in panel


        @build

            cargo help build
            cargo build [options]
            1. Build the local package and all of its dependencies:                                                                                                                     
              cargo build                                                                                                                                                           

       2. Build with optimizations:                                                                                                                                                
              cargo build --release 

         @run
            cargo run

*/

 

use std::process::Command;

use std::fs;

 

fn main() {

    // Percorso alla directory contenente gli eseguibili

    let dir_path = "c:\\CASA\\LINGUAGGI\\RUST_PROGETTI\\RUST_PURO\\RUST_PURO_COMMAND_DOS\\cmd_open_access\\exe\\";

 

    // Lista degli eseguibili da avviare in un vettore
    let executables = vec![

                            "cmd_task_kill.exe",

                            "cmd_task_kill_pid_file_testo.exe"];

 

    // Ciclo attraverso gli eseguibili

    for exe in executables {

        // Costruisci il percorso completo all'eseguibile

        let exe_path = format!("{}{}", dir_path, exe);

 

        // Controlla se l'eseguibile esiste nella directory

        if fs::metadata(&exe_path).is_ok() {

            // Esegui l'eseguibile

            match Command::new(&exe_path).status() {

                Ok(status) => {

                    if status.success() {

                        println!("{} avviato con successo.", exe);

                    } else {

                        eprintln!("Errore durante l'avvio di {}.", exe);

                    }

                }

                Err(err) => eprintln!("Errore nell'avvio di {}: {}", exe, err),

            }

        } else {

            eprintln!("L'eseguibile {} non esiste nella directory.", exe);

        }

    }

}