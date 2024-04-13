@echo off

set "outputFile=services.txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

echo Servizi attivi su Windows 10:
echo ------------------------------------------

REM Elenca i servizi attivi e stampa su video UTILIZZA NET START = utilita di rete di Windows
net start

REM Elenca i servizi attivi e salva su file
net start > "%outputFile%"

echo ------------------------------------------
echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause
