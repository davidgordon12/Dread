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
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                } 
            },
            _command => {
                let mut process = Command::new(_command)
                    .args(args)
                    .spawn()
                    .unwrap();

                let _result = process.wait().unwrap();
            }
        }
    }
}
