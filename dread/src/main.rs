use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::env;

static VERSION: f32 = 1.0;

fn main() {
    println!("Dread - Version {:.1}", VERSION);
    loop {
        print!("> ");
        let _buffer = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        
        let _command = input.trim().split_whitespace().next().unwrap();
        let args = input.trim().split_whitespace();

        match _command {
            "cd" => {
                let path = &input[3..].trim();
                println!("{}", path);
                let root = Path::new(path);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                } 
            },
            _command => {
                let mut process = Command::new("powershell")
                    .args(args)
                    .spawn()
                    .unwrap();

                let _result = process.wait().unwrap();
            }
        }
    }
}
