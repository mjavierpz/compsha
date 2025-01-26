use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use sha2::{Digest, Sha256};
use md5;
use std::{thread, time};

fn calculate_sha256(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut hasher = Sha256::new();
    hasher.update(buffer);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn calculate_md5(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = md5::compute(buffer);
    Ok(format!("{:x}", hash))
}


fn show_animation() {
    let spinner = ["-", "\\", "|", "/"];
    for i in 0..20 { // Show animation for a short time
        print!("\rCalculando... {}", spinner[i % spinner.len()]);
        std::io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
    println!("\rCalculando...        ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: compsha tipo_hash ruta_archivo hash_esperado");
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "--version" => {
            println!("compsha 0.0.1");
            std::process::exit(0);
        }
        "--help" => {
            println!("Uso: compsha <tipo_hash> <ruta_archivo> <hash_esperado>");
            println!("");
            println!("Tipos de hash habilitados:");
            println!("sha256        calcular hash SHA-256");
            println!("md5           calcular hash MD5");
            println!("");
            println!("Comandos:");
            println!("--version     versión de la aplicación");
            println!("--help        mensaje de ayuda");
            println!("Ej:");
            println!("  compsha sha256 archivo.iso hash_esperado");
            std::process::exit(0);
        }
        _ => {}
    }

    if args.len() != 4 {
        eprintln!("Uso: compsha <tipo_hash> <ruta_archivo> <hash_esperado>");
        std::process::exit(1);
    }

    let hash_type = &args[1];
    let file_path = &args[2];
    let expected_hash = &args[3];

    show_animation();

    let calculated_hash = match hash_type.as_str() {
        "sha256" => calculate_sha256(file_path),
        "md5" => calculate_md5(file_path),
        _ => {
            eprintln!("Error: Tipo de hash no soportado. Usa 'md5' o 'sha256'.");
            std::process::exit(1);
        }
    };

    match calculated_hash {
        Ok(calculated) => {
            println!("Hash O: {}", calculated);
            println!("Hash G: {}", expected_hash);
            if calculated == *expected_hash {
                println!("✓ ✓ ✓");
            } else {
                println!("✗ ✗ ✗");
            }
        }
        Err(e) => {
            eprintln!("Error al calcular el hash: {}", e);
            std::process::exit(1);
        }
    }
}

