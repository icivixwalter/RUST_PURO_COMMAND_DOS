@echo off

set "outputFile=services.txt"

rem Rimuovi il file di output se esiste già
if exist "%outputFile%" del "%outputFile%"

echo Servizi attivi su Windows 10:
echo ------------------------------------------

REM Elenca i processi e stampa su video e salva su file SOLO EDGE SE E' ATTIVO ALTRIMENTI VUOTO
for /f "tokens=2,5" %%A in ('tasklist ^| findstr /i "msedge.exe"') do (
    echo Nome: msedge.exe, PID: %%A
    echo Nome: msedge.exe, PID: %%A >> "%outputFile%"
)

echo ------------------------------------------
echo L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".

pause
