use std::env;
use std::process::{Command, ExitStatus};
use std::time::Instant;

fn main() {
    let args :Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        display("Please specify the program to be measured.");
        return;
    }

    if let Some(program) = args.get(0) {
        let program_args : Vec<String> = args[1..].to_vec();
        let mut command = command_program();
        command.arg(program);
        if let Ok(dir) = env::current_dir() {
            command.current_dir(dir);
        }
        if !program_args.is_empty() {
            command.args(program_args);
        }
        command.envs(env::vars());

        let now = Instant::now();
        let result = command.spawn();

        match result {
            Ok(mut o) => {
                if let Ok(status) = o.wait() {
                    if ExitStatus::success(&status) {
                        let msg = format!("{}\n{}", "The measurement was completed.",
                                          format!("Time: {:?}", now.elapsed()));
                        display(msg.as_str());
                    } else {
                        display("Measurement was stopped because an error occurred.");
                    }
                }
            }
            Err(_) => {
                display("Measurement was stopped because an error occurred.");
            }
        }
    }
}

fn display(msg :&str) {
    println!("\n\n{}RustEasyBench{}", "-".repeat(10), "-".repeat(10));
    println!("{}", msg);
    println!("{}", "-".repeat(33));
}

#[cfg(target_os = "windows")]
fn command_program() -> Command {
    let mut c = Command::new("powershell");
    c.arg("-Command");
    c
}

#[cfg(target_os = "linux")]
fn command_program() -> Command {
    let mut c = Command::new("sh");
    c.arg("-c");
    c
}

#[cfg(target_os = "macos")]
fn command_program() -> Command {
    let mut c = Command::new("zsh");
    c.arg("-c");
    c
}
