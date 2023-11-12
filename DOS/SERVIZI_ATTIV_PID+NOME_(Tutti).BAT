@echo off

set "outputFile=services_SERVIZI_ATTIV_PID+NOME_(Tutti).txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

@REM CREO IL FILE E CRO L'INTESTAZIONE ALLA PRIMA RIGA

	@ECHO Servizi attivi su Windows 10:					>%outputFile%"
	@ECHO.												>>"%outputFile%"
	@ECHO ------------------------------------------	>>"%outputFile%"
	@ECHO   file SERVIZI_ATTIV_PID+NOME_(Tutti).BAT		>>"%outputFile%"
	@ECHO ------------------------------------------	>>"%outputFile%"
	@ECHO.   											>>"%outputFile%"
	@ECHO.												>>"%outputFile%"

	REM Elenca i processi e stampa su video e salva su file -tutti gli attivi 
	tasklist /fi "sessionname eq console" >> "%outputFile%"
	

@REM A VIDEO SCRIVO TUTTO 
	echo Servizi attivi su Windows 10:
	echo ------------------------------------------
	echo   file SERVIZI_ATTIV_PID+NOME_(Tutti).BAT
	echo ------------------------------------------
	@REM STAMPO A VIDEOLO la lista
	tasklist /fi "sessionname eq console" 
	echo ------------------------------------------
	echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause
