
            @echo off
            set serviceName=msedge.exe

            tasklist /FI "IMAGENAME eq %serviceName%" 2>NUL | find /I "%serviceName%" >NUL

            if %errorlevel% equ 0 (
                echo Il servizio %serviceName% è attivo. Terminazione in corso...
                taskkill /F /IM %serviceName%
                echo Il servizio è stato terminato.
            ) else (
                echo Il servizio %serviceName% non è attivo.
            )

            pause