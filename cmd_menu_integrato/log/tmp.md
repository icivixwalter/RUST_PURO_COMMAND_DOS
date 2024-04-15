tmp.md



//SUB MENU UFFICIO CHROMIUM
//=====================================================================//
 fn sub_menu(sub_menu_chromiun: &str) {

    //pulisco lo schermo
        clear_screen();
    loop {
        println!("SOTTOMENU UFFICIO EDGE CHROMIUN {}",                sub_menu_chromiun);
        println!("0. Salva {}",                 sub_menu_chromiun);
        println!("1. Visualizza {}",            sub_menu_chromiun);
        println!("2. Salvataggi dei dati {}",   sub_menu_chromiun);
        println!("3. Cancella DATI {}",         sub_menu_chromiun);
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
                thread::sleep(Duration::from_secs(1)); // Ritardo di 2 secondi
                clear_screen();
                println!("Tornando al SOTTOMENU {}", sub_menu_chromiun);
            },

             //0 = salva + ritardo + ritorno sottomenu
            "1" => {
                println!("Hai selezionato 'Visualizza'");
                thread::sleep(Duration::from_secs(1)); // Ritardo di 2 secondi
                clear_screen();
                println!("Vado al SOTTOMENU EXPLORER");
                //chiamo la libreria explorer PER LA VISUALIZZAZIONE e passo come parametro clear
                explorer::lib_explorer::show_files_to_delete(clear_screen); // Chiamata alla funzione passando clear_screen come parametro


                
            }


            //2 = SALVATAGGIO DATI + ritardo + ritorno sottomenu
                "2" => {
                    println!("Hai selezionato 'SALVATAGGI DEI DATI'");
                    clear_screen();
                    println!("IL SALVATAGGIO DATI NON ATTIVO!! ");
                    println!("Torno al SOTTOMENU {}", sub_menu_chromiun);
                    thread::sleep(Duration::from_secs(1)); // Ritardo di 2 secondi
                }

            //3 = CANCELLA DATI + ritardo + ritorno sottomenu
                "3" => {
                    println!("Hai selezionato 'CANCELLAZIONE DEI DATI'");
                    clear_screen();
                    //println!("PER ORA LA CANCELLAZIONE NON E ATTIVA!! ");
                    //println!("Torno al SOTTOMENU {}", sub_menu_chromiun);
                    thread::sleep(Duration::from_secs(1)); // Ritardo di 2 secondi
                    //chiamo la libreria explorer PER LA CANCELLAZIONE e passo come parametro clear
                    explorer::lib_explorer::delete_files(clear_screen); // Chiamata alla funzione passando clear_screen come parametro


                  }


            //E= break o ritorno menu principale
            "E" | "e" => {
                println!("Uscendo dal Sottomenu...");
                thread::sleep(Duration::from_secs(1)); // Ritardo di 2 secondi
                clear_screen();
                break;
            },

            //null o scelta errata
            _ => {
                println!("SCELTA ERRATA NEL SOTTOMENU {}. RIPROVA.", sub_menu_chromiun);
                thread::sleep(Duration::from_secs(1)); // Ritardo di 5 secondi
                //@cls.02 _(chiamo la procedura)
                clear_screen();
            }


        }
    }
  }
//=====================================================================//
