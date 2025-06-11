use ctor::ctor;
use home::home_dir;
use once_cell::sync::Lazy;
use std::path;
use std::process::{Command, ExitStatus};
use std::{
    env, fmt,
    io::{Error, ErrorKind},
};

const HARB_VAR: &str = "HARBOMUX";
const TMUX_VAR: &str = "TMUX";
const HARB: Lazy<Tmux> = Lazy::new(|| Tmux::new().set_server("harbonizer").unwrap());
const TMUX: Lazy<Tmux> = Lazy::new(|| Tmux::new());
const BINARY: Lazy<String> =
    Lazy::new(|| env::current_exe().unwrap().to_str().unwrap().to_string());

enum Shell {
    Harbomux,
    Tmux,
    Default,
}

impl Clone for Shell {
    fn clone(&self) -> Self {
        match &self {
            Shell::Harbomux => Shell::Harbomux,
            Shell::Tmux => Shell::Tmux,
            Shell::Default => Shell::Default,
        }
    }
}

impl Copy for Shell {}

fn current_shell() -> Shell {
    static SHELL: Lazy<Shell> = Lazy::new(|| {
        if Os::has_env(HARB_VAR) {
            Shell::Harbomux
        } else if Os::has_env(TMUX_VAR) {
            Shell::Tmux
        } else {
            Shell::Default
        }
    });
    *SHELL
}

fn in_session_dir() -> bool {
    static VAL: Lazy<bool> = Lazy::new(|| std::path::Path::new(".harbomux").exists());
    *VAL
}

#[ctor]
fn __() {
    current_shell();
    in_session_dir();
}

enum Cmd {
    Harbour,
    Help,
    Start,
    Test,
    Hidden,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match self {
            Cmd::Harbour => "harbour",
            Cmd::Help => "help",
            Cmd::Start => "start",
            Cmd::Test => "test",
            Cmd::Hidden => "--hidden",
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
            "--hidden" => Some(Cmd::Hidden),
            _ => None,
        }
    }
}

enum HiddenCmd {
    Setup,
}

impl fmt::Display for HiddenCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match self {
            HiddenCmd::Setup => "setup",
        };
        write!(f, "{txt}")
    }
}

impl HiddenCmd {
    fn new(name: &str) -> Option<HiddenCmd> {
        match name {
            "setup" => Some(HiddenCmd::Setup),
            _ => None,
        }
    }
}

struct Os {}

impl Os {
    fn get_env(str: &str) -> Option<String> {
        env::var(str).ok()
    }

    fn set_env(key: &str, val: &str) {
        unsafe {
            env::set_var(key, val);
        }
    }

    fn has_env(var: &str) -> bool {
        Os::get_env(var).is_some()
    }

    fn cmd(args: &[&str]) -> Result<(), Error> {
        let mut cmd = Command::new(args[0]);

        for arg in &args[1..] {
            cmd.arg(arg);
        }

        cmd.output().map(|_| ())
    }

    fn cmd_ret(args: &[&str]) -> Result<String, Error> {
        let mut cmd = Command::new(args[0]);

        for arg in &args[1..] {
            cmd.arg(arg);
        }

        let output = cmd.output()?;
        if output.stderr.is_empty() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        } else {
            let err_str = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            Err(Error::new(ErrorKind::Other, err_str))
        }
    }

    fn cmd_spawn(args: &[&str]) -> Result<ExitStatus, Error> {
        let mut cmd = Command::new(args[0]);

        for arg in &args[1..] {
            cmd.arg(arg);
        }

        cmd.spawn()?.wait()
    }
}

struct Tmux {
    prefix: Option<String>,
}

impl Tmux {
    fn new() -> Tmux {
        Tmux { prefix: None }
    }

    #[allow(dead_code)]
    fn set_server(self, server: &str) -> Result<Tmux, String> {
        struct AlreadySetError;
        match self.prefix {
            Some(_) => Err("Server name is already set!".to_string()),
            None => Ok(Tmux {
                prefix: Some(server.to_string()),
            }),
        }
    }

    fn prefix(&self) -> String {
        match self.prefix {
            Some(ref server) => String::from("-L ") + server + " ",
            None => "".to_string(),
        }
    }

    #[allow(dead_code)]
    fn cmd(&self, arg: &str) -> Result<(), std::io::Error> {
        Os::cmd(&["sh", "-c", &(String::from("tmux ") + &self.prefix() + arg)])
    }

    #[allow(dead_code)]
    fn cmd_ret(&self, arg: &str) -> Result<String, Error> {
        Os::cmd_ret(&["sh", "-c", &(String::from("tmux ") + &self.prefix() + arg)])
    }

    fn cmd_spawn(&self, arg: &str) -> Result<ExitStatus, Error> {
        Os::cmd_spawn(&["sh", "-c", &(String::from("tmux ") + &self.prefix() + arg)])
    }

    fn is_on(&self) -> bool {
        self.cmd_ret("has 2>&1").is_ok_and(|s| s == "")
    }

    #[allow(dead_code)]
    fn detach_cmd(&self, arg: &str) -> Result<String, std::io::Error> {
        self.cmd_ret(&(String::from("detach -E \"") + arg + "\""))
    }

    #[allow(dead_code)]
    fn detach(&self) -> Result<String, std::io::Error> {
        self.cmd_ret("detach")
    }

    #[allow(dead_code)]
    fn new_sess_cmd(&self, arg: &str) -> Result<ExitStatus, std::io::Error> {
        self.cmd_spawn(&("new-session ".to_string() + arg))
    }

    #[allow(dead_code)]
    fn new_sess(&self) -> Result<String, std::io::Error> {
        self.cmd_ret("new-session")
    }
}

#[allow(dead_code)]
fn echo(str: &str) {
    let ret = Os::cmd_ret(&["echo", str]).unwrap();
    print!("{}\n", ret)
}

fn setup() {
    println!("setup session...")
    //run startup code
}

fn hidden_funcs() {
    let a: Vec<String> = env::args().collect();
    match HiddenCmd::new(&a[2]) {
        Some(HiddenCmd::Setup) => setup(),
        None => println!("No hidden func found!"),
    }
}

fn launch() {
    println!("TODO: launching...");
    let config_path_buf = home_dir()
        .unwrap()
        .join(".config")
        .join(HARB_VAR.to_lowercase());
    let source_config = if config_path_buf.exists() {
        format!("source {}; ", config_path_buf.to_str().unwrap())
    } else {
        String::new()
    };
    let cmd = format!(
        "{}tmux {}new-session -e {}=1",
        source_config,
        HARB.prefix(),
        HARB_VAR
    );
    Os::cmd_spawn(&["sh", "-c", &cmd]).unwrap();
    //launch tmux with env var HARBOMUX set to "pre-setup" with cmd to run binary with "--hidden
    //setup" args
}

fn harbour() {
    if Os::has_env(HARB_VAR) {
        println!("Already in harbomux!")
    } else if Os::has_env(TMUX_VAR) {
        println!("in tmux");
        TMUX.detach_cmd(&("".to_string() + &BINARY + " harbour"))
            .unwrap();
    } else if HARB.is_on() {
        HARB.cmd_spawn("attach").unwrap();
    } else {
        launch();
        println!("not in tmux")
    }
}
fn help() {
    println!("Help!");
}
fn start() {
    println!("Start!");
}
fn test() {
    println!("in harb dir: {}", in_session_dir())
}

fn fallback(args: Vec<String>) {
    println!("[{}] is not a recognized command!\n  Help:", args[1]);
    help();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_name = &args[1];

    match Cmd::new(cmd_name) {
        Some(Cmd::Harbour) => harbour(),
        Some(Cmd::Help) => help(),
        Some(Cmd::Start) => start(),
        Some(Cmd::Test) => test(),
        Some(Cmd::Hidden) => hidden_funcs(),
        None => fallback(args),
    }
}
