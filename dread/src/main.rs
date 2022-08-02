use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut process = Command::new("powershell")
            .arg("/C")
            .arg(input.trim())
            .spawn()
            .unwrap();
    }
}
