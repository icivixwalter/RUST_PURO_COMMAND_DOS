use std::io::{self, Write};
use std::process::Command;
use std::fs;

fn main() -> io::Result<()> {
    loop {
        println!("Menu:");
        println!("1) Visualizza i file da eliminare");
        println!("2) Elimina i file");
        println!("3) Esci");

        print!("Scelta: ");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => show_files_to_delete(),
            "2" => delete_files()?,
            "3" => break,
            _ => println!("Scelta non valida"),
        }
    }

    Ok(())
}

fn show_files_to_delete() {
    println!("File da eliminare:");
    let directory = "C:\\Users\\icivi\\AppData\\Local\\Microsoft\\Windows\\Explorer\\";

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                println!(" - {}", file_name);
            }
        }
    } else {
        println!("Impossibile leggere la directory");
    }
}

fn delete_files() -> io::Result<()> {
    let directory = "C:\\Users\\icivi\\AppData\\Local\\Microsoft\\Windows\\Explorer\\";

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => {
            println!("Impossibile leggere la directory");
            return Ok(());
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name.starts_with("thumbcache_") && file_name.ends_with(".db") {
                let output = Command::new("cmd")
                    .args(&["/C", "del", &format!("{}{}", directory, file_name)])
                    .output()?;
                
                if output.status.success() {
                    println!("Il file {} è stato eliminato con successo", file_name);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Errore nell'eliminazione del file {}: {}", file_name, stderr);
                }
            }
        }
    }

    Ok(())
}
