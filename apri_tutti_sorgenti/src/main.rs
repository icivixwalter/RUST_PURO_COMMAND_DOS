
/*
    RUST APRO TUTTI I SORGENTI DEL LTT
        @APRO@FILE@ESEGUIBILI@.EXE_(apro file eseguibili .exe con il @dos)
        attenzione per il terminale usa IL DOS e non 
        c'è bisogno delle dipendenze nel toml.
        Basta utilizzare il vettore per aprire file eseguibili.exe

            @dos@vettore_(apro diversi file con un vettore)
        
       SHELL
        @shell@panel
        Terminus:open default shell in panel
        per aprire la shell nel progetto digitare
        
            ctrl+shift+p
            open default shell in panel



        @build
            cargo help build
            cargo build [options] 

         
            1. Build the local package and all of its dependencies:                                                                                                                      
                                                                                                                                                                                    
              cargo build                                                                                                                                                           
                                                                                                                                                                                    
       2. Build with optimizations:                                                                                                                                                 
                                                                                                                                                                                    
              cargo build --release  


        @run
            cargo run 



    parametri:
        dir_path = C:\\CASA\\LTT\\EXE\\
*/

use std::process::Command;
use std::fs;

fn main() {
    // Percorso alla directory contenente gli eseguibili
    let dir_path = "C:\\CASA\\LTT\\EXE\\";

    // Lista degli eseguibili da avviare in un vettore
    let executables = vec![
                            "apri_sched.exe", 
                            "apri_ultime_estrazioni_xls.exe", 
                            "apri_gest_sorg.exe", 
                            "apri_ultime_estrazioni.exe"];

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
