@REM  cd.. devi posizionarti su git_desktop
CD c:\CASA\LINGUAGGI\RUST\RUST_PURO\RUST_PURO_COMMAND_DOS\
dir 

@REM  fai un controllo con status, poi aggiungi con add tutto l'albero + il comimit e il push
git status

ECHO pausa di 3 secondi per il controllo
@TIMEOUT /T 3 /NOBREAK


git pull 

@rem pausa per consentire il commit

@REM 01)
@REM sospensione per 1 secondo con crononometro 
@REM ---->	timeout /t 2 /nobreak > NUL
@REM ---->
ECHO
ECHO
ECHO  pausa di 5 secondi per consentire il commit
ECHO
@TIMEOUT /T 5 /NOBREAK
