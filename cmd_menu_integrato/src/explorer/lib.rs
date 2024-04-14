/*
    le funzioni di visualizzazione, salvataggio e cancellazione
    della directory 
    C:\Users\icivi\AppData\Local\Microsoft\Windows\Explorer\


*/
use std::fs;
use std::io;
use std::process::Command;


//pub fn show_files_to_delete() {
//visualizzo
pub fn show_files_to_delete(clear_screen: fn()) {
    
  //01= IMPOSTA I PARAMETRI
    println!("File da eliminare:");

    // Chiamata alla funzione clear_screen passata come parametro
    clear_screen();

    let directory = "C:\\Users\\icivi\\AppData\\Local\\Microsoft\\Windows\\Explorer\\";

  //02= VISUALIZZA IF
        //_____________________________________________________________________//
            if let Ok(entries) = fs::read_dir(directory) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().into_string().unwrap();
                        println!(" - {}", file_name);
                    }
                }
                    // Attendi l'input prima di proseguire + cancella
                    println!("Premi INVIO per continuare...");
                    let _ = io::stdin().read_line(&mut String::new());
                    clear_screen();

                } else {
                    println!("Impossibile leggere la directory");
                    }//else
        //_____________________________________________________________________//



  } //pub fn show_files_to_delete

//DEL.01= DELETE FILE
pub fn delete_files() -> io::Result<()> {
    
  //DEL.01.02= IMPOSTO I PARAMETRI + LEGGO LA DIRECTORY
   //___________________________________________________________________//
    let directory = "C:\\Users\\icivi\\AppData\\Local\\Microsoft\\Windows\\Explorer\\";

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => {
            println!("Impossibile leggere la directory");
            return Ok(());
          }
      };
    
  //DEL.01.03 = CICLO FOR DI CANCELLAZIONE
  //___________________________________________________________________//

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name.starts_with("thumbcache_") && file_name.ends_with(".db") {
                let output = Command::new("cmd")
                    .args(&["/C", "del", &format!("{}{}", directory, file_name)])
                    .output()?;
                
                if output.status.success() {
                    println!("Il file {} è stato eliminato con successo", file_name);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Errore nell'eliminazione del file {}: {}", file_name, stderr);
                }
            }
          }//if let Ok(entry)
      }//for entry in entries

    //*** FINE ***    
    //___________________________________________________________________//

    Ok(())
 


 } //pub fn delete_files
