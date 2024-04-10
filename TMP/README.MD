README.MD



bozza di modifica codice:


	Modificare il codice scritto in rust, pienamente funzionante, utilizzato
	per aprire una path recuperata dal parametro salvato nel file esterno denominato path_Partenza.txt.

	La modifica nro 1 deve prevedere l'utilizzo del file path_Partenza.txt come contenitore per l'elenco degli archivi da processare per il salvataggio come in questo esempio:
	
	y:\LAVORI_PUBBLICI_FINO_AL_30_09_2019\Valter\OPERE_PUBBLICHE\139_ESPROPRI\ACCESSO_ATTI_1_2016.zip
	y:\LAVORI_PUBBLICI_FINO_AL_30_09_2019\Valter\OPERE_PUBBLICHE\139_ESPROPRI\ACCESSO_ATTI_22281_2017.zip
	y:\LAVORI_PUBBLICI_FINO_AL_30_09_2019\Valter\OPERE_PUBBLICHE\139_ESPROPRI\CERT_PROC_ESPR_10135_2011.zip
	...

	La 2) modifica deve prevedere l'utilizzo di un secondo file in cui cui inserire il parametro
	della path di arrivo denominato path_Salvataggio.txt che contiene il percorso completo dove 
	salvare i file da processare esempio:
	y:\CORSETTI_MAURO\ESPROPRI\


	Questi file sono collocati nella cartella resources esempio:
	.\\resources\\path_Partenza.txt;
	.\\resources\\path_Salvataggio.txt;

	Il procedimento eseguire un ciclo recuperarndo il nome del file da processare nel primo file dei parametri
	path_Partenza.txt e sovrascrivere i file piu vecchi senza chiedere la conferma.

	Alla fine del ciclo il codice deve produrre :
		- un messaggio di avviso con l'elenco dei file salvati, 
		- aprire la path di arrivo
		- aprire la path di partenza 
	per i controlli.


	Il codice funzionante da modificare è il seguente:


	//use std::process::Command;
	use std::env;
	//use file_time::FileTime;
	use std::{fs, io::Error, path::Path, process::Command};

	use clap::Parser;


	const FILE_PARTENZA: &str = ".\\resources\\path_Partenza.txt";


	/// Simple app for backup files and folders recursively from a file with a list of paths
	#[derive(Parser, Debug)]
	#[clap(author, version, about, long_about = None)]
	struct Argomenti {
	    // short: il parametro corto (-i), long: il parametro con nome completo --input-file-with-paths
	    #[clap(short = 'i', long)]
	    input_path: String,
	}

	fn main() {
	    // leggo gli argomenti con l'iteratore arg ---> collect che vengono trasformati in un vettore
	    // let args: Vec<String> = env::args().collect();

	    //******************************* aggiunto per i parametri */
	    //se da linea di comando inserisci -i e -o prende i valori
	    //dai parametri altrimenti prende quelli dai file .txt
	    //select case con 2 bracci
	    let args = match Argomenti::try_parse() {
	        Ok(arg) => arg,
	        Err(_) => {
	            Argomenti {
	                input_path: fs::read_to_string(FILE_PARTENZA).expect("impossibile trovare il file") ,
	            }
	        }
	    }; // salva gli argomenti CLI in una nuova istanza della struct

	    //***++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
	    let path = env::current_dir().unwrap();
	    println!("Opening la cartella prevista con explorer
	            path corrente {}",path.display());
	    Command::new("explorer") // PROVA start programma...
	        //.arg( "c:\\Casa\\CDM\\LaQuercia\\La quercia finta' 30 06 2022" ) // (".") apro la cartella corrente <- Specify the directory you'd like to open.
	        .arg(&args.input_path) // (".") apro la cartella corrente <- Specify the directory you'd like to open.
	        .spawn()
	        .unwrap();
	}
