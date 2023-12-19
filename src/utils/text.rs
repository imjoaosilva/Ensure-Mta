use std::io::Write;

use colored::Colorize;

pub struct Process<W: Write> {
    server: W,
}

impl<W: Write> Process<W> {
    pub fn new(server: W) -> Process<W> {
        Process {
            server
        }
    }

    pub fn process_text(&mut self, text: &str) {
        if text.starts_with("ensure") {
            match text.trim_end() {
                "ensure help" => {
                    println!("{}", "[ENSURE] Help".green().bold());
                }

                "ensure" => {
                    println!("{}", "[ENSURE] No arguments provided.".red().bold())
                }
    
                _ => {
                    println!("{} {}", "[ENSURE] Wrong Command:".red().bold(), text.split(" ").last().unwrap_or("").white().bold())
                }
            }
        } else {

            if let Err(err) = self.server.write_all(text.as_bytes()) {
                eprintln!("Erro ao escrever no processo: {}", err);
            }
        }
    }
}
