/* ERRORI GIT
    NON_TROVA_IL_BRANCH
        questo è l'errore:
            https://www.freecodecamp.org/italian/news/git-push-to-remote-branch-how-to-push-a-local-branch-to-origin/

            fatal: You are not currently on a branch.                                                                                                                                   
            To push the history leading to the current (detached HEAD)                                                                                                                  
            state now, use                                                                                                                                                              
        comando da utilizzare                                                                                                                                                                        
         git push origin HEAD:<name-of-remote-branch> 
            con git push  con la forma base : 
                
                git push <remote> <branch>

            i parametri di base sono inseriti da git
                remote = origin (è una reposityro di GitHub)
                branch = main - branch corrente  (potrebbe essere come branche : main) (es. git push origin main)

            per vedere quale è l'attuale git remoto utilizzare:
                 git remote -v 



*/


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

fn main() {
    //HO sospeso per consentire all'exe di avere il file.txt nella directory corrente, dove si trova l'eseguibile
    //let file_path = "C:\\CASA\\LINGUAGGI\\RUST_PROGETTI\\RUST_PURO\\RUST_PURO_COMMAND_DOS\\cmd_task_kill_pid_file_testo\\pid\\elencoPid.txt"; // Sostituire con il percorso corretto del tuo file
    let file_path = "./elencoPid.txt"; // Sostituire con il percorso corretto del tuo file
  
    // Verifica se il file esiste
    if Path::new(file_path).exists() {
        // Leggi i PID dal file
        if let Ok(file) = File::open(file_path) {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(pid_str) = line {
                    if let Ok(pid) = pid_str.trim().parse::<i32>() {
                        // Costruisci il comando per inviare un segnale di kill al processo specificato
                        let output = Command::new("taskkill")
                            .args(&["/F", "/PID", &pid.to_string()])
                            .output();

                        // Verifica se il comando è stato eseguito con successo
                        match output {
                            Ok(output) => {
                                if output.status.success() {
                                    println!("Il processo con PID {} è stato terminato con successo.", pid);
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    println!("Errore durante la terminazione del processo con PID {}: {}", pid, stderr);
                                }
                            }
                            Err(err) => {
                                println!("Errore durante l'esecuzione del comando: {:?}", err);
                            }
                        }
                    } else {
                        println!("Errore nel parsing del PID: {}", pid_str);
                    }
                }
            }
        } else {
            println!("Impossibile aprire il file: {}", file_path);
        }
    } else {
        println!("Il file {} non esiste.", file_path);
    }
}
