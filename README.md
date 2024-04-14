README.MD



GIT
	ERRORI
		Errore di branch
			non riconosce in qual branch si trova:
			
			git commit -m "dos elimina file"

			fatal: cannot lock ref 'HEAD': unable to resolve reference 'refs/heads/main': reference broken  

			soluzioni
				prova con questo vedere in quale ramo ti trovi
					git branch

					git fsck
						 se da errore 

					 	 git fsck --full --no-reflogs --unreachable --lost-found

							Il comando git fsck --full --no-reflogs --unreachable --lost-found è un'opzione diagnostica per controllare l'integrità del repository Git e recuperare eventuali oggetti "persi" o "non raggiungibili". Ecco cosa fa ciascuna opzione:

							--full: Controlla l'integrità di tutti gli oggetti nel repository, incluso il controllo delle somme SHA-1.
							--no-reflogs: Esclude il controllo dei log dei riferimenti (reflogs), che sono registri di riferimenti che possono contenere riferimenti eliminati o modificati.
							--unreachable: Trova gli oggetti che non sono raggiungibili da nessun riferimento esistente nel repository.
							--lost-found: Recupera gli oggetti "persi" trovati durante il controllo. Questi oggetti sono generalmente commit, blob e alberi che non sono raggiungibili da nessun riferimento.
							Quindi, quando esegui il comando git fsck --full --no-reflogs --unreachable --lost-found, Git controllerà l'integrità del repository, troverà gli oggetti "persi" o "non raggiungibili" e li metterà nella directory .git/lost-found/. Puoi esaminare questi oggetti e decidere se sono importanti o meno. Se ritieni che siano importanti, puoi rinominarli e reintegrarli nel repository manualmente.

							Dopo aver eseguito questo comando, potresti voler ripetere il comando git fsck per vedere se ci sono ancora problemi rimanenti nel repository. Se ci sono ancora errori, potrebbe essere necessario indagare ulteriormente per risolverli.










					git init
						se non funziona il comando  git fsck  cancella la cartella locale .git e reinizializza git senza cancellare
						i dati in locale con 

						A) git init

							risultato
							Reinitialized existing Git repository in C:/CASA/LINGUAGGI/RUST_PROGETTI/RUST_PURO/RUST_PURO_COMMAND_DOS/TMP/.git/

							Il messaggio "Reinitialized existing Git repository" indica che è stata riinizializzata una repository Git esistente. Questo significa che c'era già una repository Git presente nella cartella, e il comando git init ha semplicemente reinizializzato questa repository

						B) git remote add
							In questo caso, poiché hai già una repository Git nella cartella, puoi procedere direttamente ad aggiungere il repository remoto utilizzando il comando git remote add, come discusso in precedenza:

							git remote add origin https://github.com/icivixwalter/RUST_PURO_COMMAND_DOS.git

						C) ramo principale
							una inizializzato la repository di trovi sul ramo
							principale detto master es * master.
							(vedi con: git branch)
							Puoi lavorare anche su un ramo sencondario da creare
							con git checkout -b sviluppo e poi salvare a conclusione sul quello principale.
							Prima lavori su quello di sviluppo e dopo quando tutto è a posto su quello principale.
								esegui: git checkout -b sviluppo


							attivo la procedura dos di ripetizione del comando per le 10 direcotry:

							for /f "delims=" %i in (directories_to_add.txt) do git add "%i"

							questo comando legge le righe dal file ed esegue add di git sulle path 
							relative memoerizzate.



						D) RIEPILOGO REINIZIALIZZA REPOSITORI LOCALE:
							1) Prima cancelli il .git dal locale
							2) apri shell dos
							3) git init   						= inizializzi la repository locale
							4) git add .						= aggiungi tutti i dati all'area di staging
							5) git commit -m "?"				= eseguo il commit 
							6) git branch						= controllo se esiste il master
							7) git checkout -b sviluppo			= creo un ramo secondario e si posiziona
							8) creo il file .txt				= nel file salvo tutte le directory
							9) for /f "delims=" %i 
								in (directories_to_add.txt) 
								do git add 
								"%i"								=  eseguo il for dei comandi add per ogni file

							10) git checkout master					= passo al master
							11) git merge sviluppo					= faccio il merge tra sviluppo --> 
																	master

							12) git commit -m "Merge branch 'sviluppo' into 'master'"
							13) sul master fare il git add . + git commit -m "agg"
							14) per configuare su github il branch master eseguire:
								git push --set-upstream origin master

								configura la repository remota da chiamare origin master

								









