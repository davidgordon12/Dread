use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::process::Command;
use std::path::Path;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::env;
use whoami;

static VERSION: f32 = 1.0;

fn main() {
    loop {
        print_header();
        
        if let Event::Key(event) = event::read().expect("Error reading key") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => break,
                _ => {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    
                    let _command = input.trim().split_whitespace().next().unwrap();
                    let args = input.trim().split_whitespace();

        
                    match _command {
                        "cd" => {
                            change_directory(&input);
                        },
                        _command => {
                            if cfg!(target_os = "windows"){
                                let mut process = Command::new("powershell")
                                    .args(args)
                                    .spawn()
                                    .unwrap();
                                let _result = process.wait().unwrap();
                            } else {
                                let mut process = Command::new("sh")
                                    .args(args)
                                    .spawn()
                                    .unwrap();
                                let _result = process.wait().unwrap();
                            }
                        }
                    }
                }
            }
            println!("{:?}", event);
        }
    }
}

fn change_directory(input: &str) {
    let path = &input[3..].trim();
    let root = Path::new(path);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

fn get_directory() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn print_header() {
    print!(" {0} at ", whoami::username());
    get_directory();
    print!(" > ");
    let _buffer = stdout().flush();
}