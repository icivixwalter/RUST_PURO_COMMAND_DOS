
use std::io::{self, Write};     //imput
//ritardo schermo
use std::thread;
use std::time::Duration;

//@cls.00_(libreria per cls dello schermo)
use std::process::Command;

// Importa il modulo explorer
pub mod explorer{
   pub mod lib;
}





// Funzione per il menu principale
fn main_menu() {
       //pulisco lo schermo
        clear_screen();
     println!("APRO PROCEDURA MENU IN RUST");

    loop {
        println!("MENU PRINCIPALE");
        println!("0. SOTTOMENU EXPLORER");
        println!("1. SOTTOMENU SALVATAGGI");
        println!("2. SOTTOMENU DA UTILIZZARE");
        println!("X. ESCI");

        print!("Seleziona un'opzione: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Errore durante la lettura dell'input.");

        match choice.trim() {
            "0" => sub_menu("EXPLORER"),
            "1" => sub_menu("SALVATAGGI"),
            "2" => sub_menu("DA UTILIZZARE"),
            "X" | "x" => break,
            _ => {
                println!("SCELTA ERRATA NEL MENU PRINCIPALE. RIPROVA.");
                thread::sleep(Duration::from_secs(2)); // Ritardo di 5 secondi
                //@cls.02 _(chiamo la procedura)
                clear_screen();
            }

        }
    }
}

// Funzione per i sottomenu
fn sub_menu(sub_menu_name: &str) {

    //pulisco lo schermo
        clear_screen();
    loop {
        println!("SOTTOMENU {}", sub_menu_name);
        println!("0. Salva");
        println!("1. Visualizza");
        println!("2. Salvataggi dei dati");
        println!("3. Cancella DATI");
        println!("E. Torna al menu principale");

        print!("Seleziona un'opzione: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        //controllo per errore di lettura
        io::stdin().read_line(&mut choice).expect("Errore durante la lettura dell'input.");

        match choice.trim() {
            //0 = salva + ritardo + ritorno sottomenu
            "0" => {
                println!("Hai selezionato 'Salva'");
                thread::sleep(Duration::from_secs(2)); // Ritardo di 2 secondi
                clear_screen();
                println!("Tornando al SOTTOMENU {}", sub_menu_name);
            },

             //0 = salva + ritardo + ritorno sottomenu
            "1" => {
                println!("Hai selezionato 'Visualizza'");
                thread::sleep(Duration::from_secs(2)); // Ritardo di 2 secondi
                clear_screen();
                //println!("Tornando al SOTTOMENU {}", sub_menu_name);
                //chiamo la funzione explorer    
                //explorer::lib::show_files_to_delete(); // Chiamata alla funzione per visualizzare i file da eliminare
                //passo come parametro clear
                explorer::lib::show_files_to_delete(clear_screen); // Chiamata alla funzione passando clear_screen come parametro


                
            }


            //2 = SALVATAGGIO DATI + ritardo + ritorno sottomenu
                "2" => {
                    println!("Hai selezionato 'SALVATAGGI DEI DATI'");
                    clear_screen();
                    println!("IL SALVATAGGIO DATI NON ATTIVO!! ");
                    println!("Torno al SOTTOMENU {}", sub_menu_name);
                    thread::sleep(Duration::from_secs(2)); // Ritardo di 2 secondi
                }

            //3 = CANCELLA DATI + ritardo + ritorno sottomenu
                "3" => {
                    println!("Hai selezionato 'CANCELLAZIONE DEI DATI'");
                    clear_screen();
                    println!("PER ORA LA CANCELLAZIONE NON E ATTIVA!! ");
                    println!("Torno al SOTTOMENU {}", sub_menu_name);
                    thread::sleep(Duration::from_secs(2)); // Ritardo di 2 secondi
                  }


            //E= break o ritorno menu principale
            "E" | "e" => {
                println!("Uscendo dal Sottomenu...");
                thread::sleep(Duration::from_secs(2)); // Ritardo di 2 secondi
                clear_screen();
                break;
            },

            //null o scelta errata
            _ => {
                println!("SCELTA ERRATA NEL SOTTOMENU {}. RIPROVA.", sub_menu_name);
                thread::sleep(Duration::from_secs(2)); // Ritardo di 5 secondi
                //@cls.02 _(chiamo la procedura)
                clear_screen();
            }


        }
    }
}


// Funzione per pulire lo schermo - @cls.01_(costruisco cls in windows, dos o shell)
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }
}

fn main() {
    main_menu();
}
