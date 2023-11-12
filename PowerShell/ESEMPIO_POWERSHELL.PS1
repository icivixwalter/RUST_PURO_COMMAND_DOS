//ESEMPIO_POWERSHELL.BAT

# Ottieni i processi di Microsoft Edge
$edgeProcesses = Get-Process -Name msedge -ErrorAction SilentlyContinue

# Verifica se ci sono processi di Microsoft Edge
if ($edgeProcesses) {
    Write-Host "Elenco dei processi di Microsoft Edge:"
    
    # Scorro l'array di processi di Microsoft Edge e stampo PID e Nome del processo
    foreach ($process in $edgeProcesses) {
        Write-Host "PID: $($process.Id), Nome: $($process.ProcessName)"
    }
} else {
    Write-Host "Nessun processo di Microsoft Edge in esecuzione."
}

# Aggiungi una pausa per mantenere aperto il terminale
Start-Sleep -Seconds 30




