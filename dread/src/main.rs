use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

static VERSION: f32 = 1.0;

fn main() {
    println!("Dread - Version {:.1}", VERSION);
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut process = Command::new("powershell")
            .arg(input.trim())
            .spawn()
            .unwrap();

        let _result = process.wait().unwrap();
    }
}
