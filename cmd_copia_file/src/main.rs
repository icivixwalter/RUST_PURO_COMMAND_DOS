/* SALVATAGGIO FILE CON IL COMANDO DOS XCOPY

    @01 = lo standard io 
    @02 = funzione processa file con for
        legge il contenuto dal file di partenza passato come parametro
        
        @01.02 = Esegui XCOPY per copiare il file
            crea una istanza del command ed esegue XCOPY passando 2 parametri
            CON LA FUNZIONE fs::read_to_string_(@legge i file ed il suo contenuto e le stringhe ed utilizza
                                                il @file@sistem)
            input_file  = funzione .lines che legge la singola riga del file e lo
                    libera di spazi
            output_file = FORMATTA SIA OTUPUT CHE IMPUT FILE

        //@03.00 = main

            @03.01 = imposta la path dei due parametri
                prima di compilare devi confermare o cabiare questa
                path relativa ai due file dei parametri altrimenti non
                funziona il salvataggio
            
            @03.02 = path da cambiare
            path da cambiare o confermare
            y:/LAVORI_PUBBLICI_FINO_AL_30_09_2019/Valter/OPERE_PUBBLICHE/139_ESPROPRI/RUST_COPIA_FILE/resources/

            
            @03.03 = funzione fs::read_to_string per la lettura del file
                viene utilizzata per aprire il file e leggere le singole
                righe di testo da cui ricare la path di lettura e di salvataggio.
*/

/* 
    @01
*/
use std::fs;

//@02 = funzione processa file con for
fn process_files(input_file_list: &str, output_dir: &str) -> Result<(), std::io::Error> {
    // Leggi il contenuto del file che contiene i percorsi dei file da copiare
    let file_list_content = fs::read_to_string(input_file_list)?;
    for line in file_list_content.lines() {
        let input_file = line.trim();
        let output_file = format!("{}/{}", output_dir, input_file);

        // @01.02 = Esegui XCOPY per copiare il file
        let output = std::process::Command::new("cmd")
            .args(&["/C", "XCOPY", input_file, &output_dir, "/Y"]) // /Y sovrascrive il file di destinazione senza chiedere conferma
            .output()?;
        
        // Stampa l'output del comando XCOPY per il controllo
        println!("\n\n Output del comando XCOPY: {:?}", output);
        
        println!("\n\n File copiato da {} a {}", input_file, output_file);
    }
    Ok(())
}

//@03.00 = main
fn main() {
    println!("SALVATAGGIO FILE CON PARAMETRI!");

    /* E' stato modificato il percorso dei parametri, non piu dalla cartella risorse di rust
       ma da una cartella specifica che in questo caso si trova su y  ma puo essere imposta
       da qualunque altra parte.
    */


    // Percorso del file contenente l'elenco dei file da copiare
    //modifico metto la path di y:    
    //let input_file_list_path = "./resources/path_Partenza.txt";
        let input_file_list_path ="y:/LAVORI_PUBBLICI_FINO_AL_30_09_2019/Valter/OPERE_PUBBLICHE/139_ESPROPRI/RUST_COPIA_FILE/resources/path_Partenza.txt";
    
    
    //@03.01 = imposta la path dei due parametri
    // Percorso della directory di output
    //let output_dir_path = "./resources/path_Salvataggio.txt";

        //@03.02 = path da cambiare
        let output_dir_path ="y:/LAVORI_PUBBLICI_FINO_AL_30_09_2019/Valter/OPERE_PUBBLICHE/139_ESPROPRI/RUST_COPIA_FILE/resources/path_Salvataggio.txt";

        //@03.03 = funzione fs::read_to_string per la lettura del file
        // Leggi il percorso della directory di output da path_Salvataggio.txt
        let output_dir_content = fs::read_to_string(output_dir_path)
            .expect("Impossibile leggere il file path_Salvataggio.txt");
        let output_dir = output_dir_content.trim();

        // Stampa il percorso della directory di output letto dal file
        println!("\n\n Output directory letto dal file: {}", output_dir);

    // Processa i file utilizzando i parametri letti dai file
    if let Err(err) = process_files(input_file_list_path, output_dir) {
        eprintln!("Errore durante l'elaborazione dei file: {}", err);
    }
}
