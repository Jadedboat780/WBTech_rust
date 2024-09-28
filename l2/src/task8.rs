use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command, Stdio};

fn main() {
    loop {
        // Ожидание ввода
        print!("shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Распределение команд по каналам
        let commands = input
            .split('|')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        if commands.len() > 1 {
            handle_pipes(commands);
        } else {
            handle_command(input.to_string());
        }
    }
}

/// Обработка команд с помощью pipes
fn handle_pipes(commands: Vec<String>) {
    let mut prev_output = None;

    for (i, cmd) in commands.iter().enumerate() {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let (command, args) = (parts[0], &parts[1..]);

        let mut child = Command::new(command)
            .args(args)
            .stdin(prev_output.take().map_or(Stdio::inherit(), Stdio::from))
            .stdout(if i == commands.len() - 1 {
                Stdio::inherit()
            } else {
                Stdio::piped()
            })
            .spawn()
            .unwrap_or_else(|_| {
                eprintln!("Failed to execute command: {}", command);
                process::exit(1);
            });

        if i < commands.len() - 1 {
            prev_output = Some(child.stdout.take().unwrap());
        }

        child.wait().unwrap();
    }
}

/// Обработка команд
fn handle_command(input: String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];

    match command {
        "cd" => change_directory(args),
        "pwd" => print_working_directory(),
        "echo" => echo(args),
        "kill" => kill_process(args),
        "ps" => print_processes(),
        "exit" => process::exit(0),
        _ => execute_external_command(command, args),
    }
}

/// Команда cd
fn change_directory(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cd: missing argument");
        return;
    }

    let new_dir = args[0];
    if let Err(err) = env::set_current_dir(Path::new(new_dir)) {
        eprintln!("cd: {}", err);
    }
}

/// Команда pwd
fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: {}", err),
    }
}

/// Команда echo
fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

/// Команда kill
fn kill_process(args: &[&str]) {
    if args.is_empty() {
        eprintln!("kill: missing argument");
        return;
    }

    let pid: u32 = args[0].parse().unwrap_or_else(|_| {
        eprintln!("kill: invalid pid");
        process::exit(1);
    });

    match process::id().checked_sub(pid) {
        Some(_) => {
            if let Err(err) = Command::new("kill").arg(pid.to_string()).status() {
                eprintln!("kill: failed to kill process {}: {}", pid, err);
            }
        }
        None => eprintln!("kill: process with pid {} not found", pid),
    }
}

/// Команда ps
fn print_processes() {
    let output = Command::new("ps")
        .arg("-eo")
        .arg("pid,comm,etime")
        .output()
        .expect("failed to execute ps");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        eprintln!("ps: failed to get processes");
    }
}

/// Выполнение внешних команд из via/forkexec
fn execute_external_command(command: &str, args: &[&str]) {
    match Command::new(command).args(args).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("{}: command failed with status {}", command, status);
            }
        }
        Err(err) => eprintln!("{}: failed to execute: {}", command, err),
    }
}
