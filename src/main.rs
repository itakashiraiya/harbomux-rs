use std::process::Command;
use std::{env, fmt};

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

    #[allow(dead_code)]
    fn set_env(key: &str, val: &str) {
        unsafe {
            env::set_var(key, val);
        }
    }

    fn has_env(var: &str) -> bool {
        Os::get_env(var).is_some()
    }

    fn cmd(str: &str, args: &[&str]) -> String {
        let mut child = Command::new(str);

        for arg in args {
            child.arg(arg);
        }

        let out = child.output().expect("Failed cmd");

        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }
}

const HARB_VAR: &str = "HARBOMUX";
const TMUX_VAR: &str = "TMUX";

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
            Some(ref server) => String::from("-L ") + server,
            None => "".to_string(),
        }
    }

    fn cmd(&self, args: &[&str]) -> String {
        let mut child = Command::new("sh");

        let arg = args.join(" ");

        let prefix = &self.prefix();

        let arg = format!("tmux {} {}", prefix, arg);

        println!("tmux arg: {}", arg);

        child.arg("-c");
        child.arg(arg);

        let out = child.output().expect("Failed cmd");

        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

    fn new_session(&self) -> String {
        self.cmd(&["new-session", "-d"])
    }
}

#[allow(dead_code)]
fn echo(str: &str) {
    let ret = Os::cmd("echo", &[str]);
    print!("{}\n", ret)
}

fn setup() {
    if Os::get_env(HARB_VAR).as_deref() != Some("pre-setup") {
        println!("Cant run setup here!");
        return;
    }
    Os::set_env(HARB_VAR, "1");
    //run startup code
}

fn hidden_funcs() {
    let a: Vec<String> = env::args().collect();
    match HiddenCmd::new(&a[2]) {
        Some(HiddenCmd::Setup) => setup(),
        None => println!("No hidded func found!"),
    }
}

fn launch() {
    //launch tmux with env var HARBOMUX set to "pre-setup" with cmd to run binary with "--hidden
    //setup" args
}

// static TMUX: once_cell::sync::Lazy<Tmux> =
//     once_cell::sync::Lazy::new(|| Tmux::new().set_server("harbonizer").unwrap());

#[allow(non_snake_case)]
fn TMUX() -> &'static Tmux {
    use once_cell::sync::Lazy;
    static TMUX: Lazy<Tmux> = Lazy::new(|| Tmux::new().set_server("harbonizer").unwrap());
    &TMUX
}

#[allow(non_snake_case)]
fn BINARY() -> &'static String {
    use once_cell::sync::Lazy;
    static BINARY: Lazy<String> = Lazy::new(|| {
        env::current_exe()
            .expect("???")
            .to_str()
            .expect("???")
            .to_string()
    });
    &BINARY
}

fn load() {}

fn harbour() {
    println!("{}", BINARY());
    if Os::has_env(TMUX_VAR) {
        //detach with binary cmd "harbour"
    } else {
        launch();
        load();
        println!("not in tmux")
    }
}
fn help() {
    println!("help");
}
fn start() {
    println!("start");
}
fn test() {
    let res = Tmux::cmd(&TMUX(), &["ls", "-F", "#S"]);
    let sess = TMUX().new_session();
    println!("sess: {}", sess);
    println!("2:\n   {}", res);
    let var = Os::get_env(TMUX_VAR).unwrap();
    println!("tmux: {}", var);
}

fn fallback() {
    println!("fallback");
    help();
}

fn main() {
    println!("{}", BINARY());
    let args: Vec<String> = env::args().collect();
    let cmd_name = &args[1];

    match Cmd::new(cmd_name) {
        Some(Cmd::Harbour) => harbour(),
        Some(Cmd::Help) => help(),
        Some(Cmd::Start) => start(),
        Some(Cmd::Test) => test(),
        Some(Cmd::Hidden) => hidden_funcs(),
        None => fallback(),
    }
}
