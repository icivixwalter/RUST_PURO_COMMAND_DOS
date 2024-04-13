@echo off

set "outputFile=services.txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

echo Servizi attivi su Windows 10:
echo ------------------------------------------

REM Elenca i servizi attivi con PID, STATO e GRUPPO e stampa su video
tasklist /svc | findstr /i "NomeServizio PID Stato Gruppo"

REM Elenca i servizi attivi e salva su file
tasklist /svc > "%outputFile%"

echo ------------------------------------------
echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause












goto primo 
@echo off

set "outputFile=services.txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

echo Servizi attivi su Windows 10:
echo ------------------------------------------

REM Elenca i servizi attivi con PID, STATO e GRUPPO e stampa su video
tasklist /svc | findstr /i "NomeServizio PID Stato Gruppo"

REM Elenca i servizi attivi e salva su file
tasklist /svc > "%outputFile%"

echo ------------------------------------------
echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause

:primo