/*lib_ufficio_edge_chronium_cache_internet.rs 
	libreia per il cache edge dell'ufficio

	  le funzioni di visualizzazione, salvataggio e cancellazione
    della directory 
    C:\\Users\\walter.rossi\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache\\Cache_Data\\


    file f_0*.*


*/




//00_@explorer_LIBRERIE
//-----------------------------------------------------------------------------------//
    use std::fs;
    use std::io;
    use std::process::Command;
  //-----------------------------------------------------------------------------------//

//01_@explorer_FUNZIONE_show_(visualizzo i file prima di cancellare)
//-----------------------------------------------------------------------------------//

        pub fn show_files_to_delete(_clear_screen: fn()) {
        
      //01= IMPOSTA I PARAMETRI
        println!("File da eliminare:");

        // Chiamata alla funzione _clear_screen passata come parametro
        _clear_screen();

        let directory = "C:\\Users\\walter.rossi\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache\\Cache_Data\\";

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
                        _clear_screen();

                    } else {
                        println!("Impossibile leggere la directory");
                        }//else
            //_____________________________________________________________________//



      } //pub fn show_files_to_delete
    //-----------------------------------------------------------------------------------//


//03_@explorer_FUNZIONE_DELETE_(cancello tutti i  FILE)
//CANCELLO I FILE
//-----------------------------------------------------------------------------------//
  pub fn delete_files(_clear_screen: fn()) -> io::Result<()> {
    
    //DEL.01.02= IMPOSTO I PARAMETRI + LEGGO LA DIRECTORY
     //___________________________________________________________________//
      let directory = "C:\\Users\\walter.rossi\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache\\Cache_Data\\";

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

              if file_name.starts_with("f_0") && file_name.ends_with("") {
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

                      // Attendi l'input prima di proseguire + cancella
                      println!("Premi INVIO per continuare...");
                      let _ = io::stdin().read_line(&mut String::new());
                      _clear_screen();

      Ok(())

    } //pub fn delete_files

   //-----------------------------------------------------------------------------------//