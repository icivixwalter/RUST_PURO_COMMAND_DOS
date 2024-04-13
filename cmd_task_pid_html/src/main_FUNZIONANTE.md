//Assicurati di eseguire cargo run dopo aver 
//aggiornato il tuo progetto Rust con queste modifiche. 
//Il server sarà accessibile all'indirizzo http://127.0.0.1:3030/.



/*
    OPERAZIONI
        1) fai partire il file .exe
        2) apri edge
        3) clicca sull'indirizzo web: http://127.0.0.1:3030/.



*/


// funziona 2 colonne
use warp::Filter;
//
#[tokio::main]
async fn main() {
    // Definisci un filtro che gestirà la richiesta GET a /
    let routes = warp::path::end().map(|| warp::reply::html(render_services_table()));

    // Avvia il @server sulla @porta 3030, @costruisce_(@indirizzo @http)
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
//
//// Funzione per generare il codice HTML con la tabella dei servizi
fn render_services_table() -> String {
    // Esegui il comando tasklist e ottieni l'output come stringa
    let output = std::process::Command::new("tasklist")
        .arg("/fi")
        .arg("sessionname eq console")
        .output()
        .expect("Failed to execute command");

    // Verifica se il comando è stato eseguito correttamente
    if output.status.success() {
        // Converte l'output in una stringa UTF-8
        let result = String::from_utf8_lossy(&output.stdout);

        // Costruisci il codice HTML con la tabella dei servizi
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Servizi Windows</title>
            </head>
            <body>
                <h1>Servizi attivi su Windows 10</h1>
                <table border="1">
                    <tr>
                        <th>Nome del Servizio</th>
                        <th>PID</th>
                        <th>nome sessione</th>
                        
                    </tr>
                    {}
                </table>
                <p>L'elenco dei servizi attivi è stato salvato nel file "%outputFile%".</p>
            </body>
            </html>
        "#,
            // Inserisci i dati della tabella qui
            result.lines()
                .skip(3) // Salta le prime 3 righe dell'output
                .filter(|line| !line.trim().is_empty()) // Rimuovi le righe vuote
                .map(|line| {
                    let fields: Vec<&str> = line.trim().split_whitespace().collect();
                    format!(
                        //costruisco 3 colonne    
                        "<tr><td>{} 
                        </td><td>{} 
                        </td><td>{} 

                        </td></tr>",
                        fields[0],      // 1 colonna = Nome del Servizio 
                        fields[1],      // 2 colonna = PID (..ATTENDO ALLA VIRGOLA ,)
                        fields[2]       // 3 colonna = nome sessione   (NO VIRGOLA , )
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    } else {
        // Messaggio di errore se il comando non ha avuto successo
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Errore</title>
            </head>
            <body>
                <h1>Errore durante l'esecuzione del comando</h1>
                <p>Dettagli dell'errore: {:?}</p>
            </body>
            </html>
        "#,
            output.status
        )
    }
}




























