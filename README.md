# RUST_PURO_COMMAND_DOS - @README@HELP

COMANDI_GIT+DOS+RUST

	COMANDI_RUST
	
	C
		cargo
			crea un nuovo progetto
				cargo init [Option] [path]

					@cargo@esempio_(@crea un progetto con cargo in @binario)
						cargo init cmd_task_Pid --bin

				Option
					-q, --quiet                Do not print cargo log messages

					--bin                  Use a binary (application) template [default]
					  							crea una applicazione binario
					-h, --help                 Print help 
													@help_(di cargo)
	H
		help
			cargo init --help


	CLONE
		git clone https://github.com/icivixwalter/RUST_PURO_COMMAND_DOS.git



	P
		PATH

			@path@pid_(si trova qui:)
				c:\CASA\LINGUAGGI\RUST_PROGETTI\RUST_PURO\ 

PROGETTI_CREATI
	cmd_task_Kill
		CANCELLA I PROCESSI
	cmd_task_Pid
		controlla i pid di windows, è necessaria la libreia nel toml:
			winapi = { version = "0.3", features = ["winuser"] }
   help_html
      crea una file @html o una @libreria@help@html
         vai al codice---> @help@documentazione