use std::{fs, process::{Command, Stdio}};
use std::io::{self, BufRead, Write};
use colored::Colorize;

use crate::utils::text;

pub fn start() {
    match get_mta() {
        Ok(result) => {
            println!("{} : {} 🎉", "MTA SERVER.exe Found".green().bold(), result.cyan());
            println!("{}", "Starting process...".green());
            start_mta(result)
        }
        Err(e) => {
            eprint!("{}: {}", "Error".red().bold(), e.red())
        }
    }
}

fn start_mta(path: String) {
    let mut server_process = match Command::new(path)
    .stdin(Stdio::piped())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn() {
        Ok(process) => process,
        Err(err) => {
            eprintln!("Erro ao iniciar o processo: {}", err);
            return;
        }
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let server_stdin = match server_process.stdin.take() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Não foi possível acessar a entrada padrão do processo.");
            return;
        }
    };

    let mut input = String::new();

    let mut process = text::Process::new(server_stdin);
    loop {
        io::stdout().flush().expect("Falha ao limpar o buffer de saída.");
        input.clear();

        if let Err(err) = reader.read_line(&mut input) {
            eprintln!("Erro ao ler a entrada: {}", err);
            break;
        }

        process.process_text(&input);
        
    }
}

fn get_mta() -> Result<String, String> {
    if let Ok(directory) = std::env::current_dir() {
        match fs::read_dir(directory) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(file) = entry {
                        if file.file_name() == "MTA Server.exe" {
                            if let Some(path) = file.path().parent() {
                                return Ok(format!("{}\\MTA Server.exe", path.to_path_buf().to_string_lossy().into_owned()));
                            }
                        }
                    }
                }

                return Err(String::from("MTA SERVER.exe Not found! (Make sure the executable is in the correct folder)"));
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    Err(String::from("Failed to get current directory!"))
}