@echo off

set "outputFile=services.txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

echo Servizi attivi su Windows 10:
echo ------------------------------------------

REM Elenca i servizi attivi con informazioni dettagliate (NomeServizio, PID, Stato, Gruppo, Utente) e stampa su video
tasklist /svc | findstr /i /v/m "NomeServizio PID Stato Gruppo"

REM Elenca i servizi attivi e salva su file
tasklist /svc /v /i /m > "%outputFile%"

echo ------------------------------------------
echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause







