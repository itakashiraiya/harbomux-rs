use std::process::Command;
use std::time::Duration;
use std::{env, fmt};

enum Cmd {
    Harbour,
    Help,
    Start,
    Test,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match self {
            Cmd::Harbour => "harbour",
            Cmd::Help => "help",
            Cmd::Start => "start",
            Cmd::Test => "test",
        };
        write!(f, "{txt}")
    }
}

impl Cmd {
    fn new(str: &str) -> Option<Cmd> {
        match str {
            "harbour" => Some(Cmd::Harbour),
            "help" => Some(Cmd::Help),
            "start" => Some(Cmd::Start),
            "test" => Some(Cmd::Test),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let cmd_name = stringify!(args[1]);
    let cmd_name = &args[1];

    match Cmd::new(cmd_name) {
        Some(Cmd::Harbour) => harbour(),
        Some(Cmd::Help) => help(),
        Some(Cmd::Start) => start(),
        Some(Cmd::Test) => test(),
        None => fallback(),
    }
}

fn echo() {
    let child = Command::new("sh")
        .arg("-c")
        .arg("ls")
        .output()
        .expect("Failed cmd");

    print!("{}\n", String::from_utf8_lossy(&child.stdout));
}

fn harbour() {
    echo();
    std::thread::sleep(Duration::from_secs(2));
    println!("harbour");
}
fn help() {
    println!("help");
}
fn start() {
    println!("start");
}
fn test() {
    println!("test");
}
fn fallback() {
    println!("fallback");
    help();
}
