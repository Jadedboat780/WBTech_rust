use std::env;
use std::fs::File;
use std::io::{self, BufRead, Stdin, Stdout, Write};
use std::path::PathBuf;
use std::process::{self, Command, Stdio};

fn main() {
    let cli = Cli::new();
    cli.run()
}

/// Интерпретатор командной строки
struct Cli {
    stdin: Stdin,
    stdout: Stdout,
}

impl Cli {
    /// Конструктор
    pub fn new() -> Self {
        let stdin = io::stdin();
        let stdout = io::stdout();

        Cli { stdin, stdout }
    }

    /// Запуск CLI
    pub fn run(&self) {
        loop {
            self.print_prompt();

            let input = self.read_input();

            if let Err(e) = self.execute(&input) {
                writeln!(self.stdout.lock(), "Ошибка: {}", e).unwrap();
            }
        }
    }

    /// Чтение пользовательского ввода
    fn read_input(&self) -> String {
        let mut input = String::new();
        self.stdin.lock().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    /// Печать начального символа строки
    fn print_prompt(&self) {
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    /// Запуск команды
    fn execute(&self, command: &str) -> io::Result<()> {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args= &parts.collect::<Vec<&str>>();

        match cmd {
            "cd" => self.cd(args),
            "ls" => self.ls(args),
            "echo" => self.echo(args),
            "pwd" => self.pwd(),
            "exit" => process::exit(0),
            "" => Ok(()),
            _ => self.run_external_command(cmd, &args),
        }
    }

    /// Команда cd
    fn cd(&self, args: &[&str]) -> io::Result<()> {
        let dir = if args.is_empty() {
            env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"))
        } else {
            PathBuf::from(args[0])
        };

        env::set_current_dir(dir)?;
        Ok(())
    }

    /// Команда ls
    fn ls(&self, args: &[&str]) -> io::Result<()> {
        let dir = if args.is_empty() { "." } else { args[0] };
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            println!("{}", entry.file_name().to_string_lossy());
        }
        Ok(())
    }

    /// Команда echo
    fn echo(&self, args: &[&str]) -> io::Result<()> {
        println!("{}", args.join(" "));
        Ok(())
    }

    /// Команда pwd
    fn pwd(&self) -> io::Result<()> {
        let current_dir = env::current_dir()?;
        println!("{}", current_dir.display());
        Ok(())
    }

    /// Выполнение внешних команд с поддержкой редиректов и пайплайнов
    fn run_external_command(&self, cmd: &str, args: &[&str]) -> io::Result<()> {
        let commands = self.parse_pipeline(cmd, args);
        let mut prev_command_stdout = None;

        for (cmd, args, input, output, append) in commands {
            let stdin = match input {
                Some(file) => Stdio::from(File::open(file)?),
                None => match prev_command_stdout.take() {
                    Some(output) => Stdio::from(output),
                    None => Stdio::inherit(),
                },
            };

            let stdout = if let Some(file) = output {
                if append {
                    Stdio::from(File::options().append(true).open(file)?)
                } else {
                    Stdio::from(File::create(file)?)
                }
            } else {
                Stdio::piped()
            };

            let mut child = Command::new(cmd)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn()?;

            // Получаем ChildStdout, который можно использовать в следующей итерации.
            prev_command_stdout = child.stdout.take();
        }

        // Если последняя команда имеет вывод, копируем его в стандартный поток вывода
        if let Some(mut final_stdout) = prev_command_stdout {
            io::copy(&mut final_stdout, &mut io::stdout())?;
        }

        Ok(())
    }

    /// Парсинг команды с учетом пайплайнов и редиректов
    fn parse_pipeline<'a>(
        &self,
        cmd: &'a str,
        args: &'a [&str],
    ) -> Vec<(
        &'a str,
        Vec<&'a str>,
        Option<&'a str>,
        Option<&'a str>,
        bool,
    )> {
        let mut commands = Vec::new();
        let mut command = cmd;
        let mut command_args = Vec::new();
        let mut input = None;
        let mut output = None;
        let mut append = false;

        for &arg in args {
            match arg {
                "<" => input = Some(self.next_arg(args)),
                ">" => {
                    output = Some(self.next_arg(args));
                    append = false;
                }
                ">>" => {
                    output = Some(self.next_arg(args));
                    append = true;
                }
                "|" => {
                    commands.push((command, command_args, input, output, append));
                    command = self.next_arg(args);
                    command_args = Vec::new();
                    input = None;
                    output = None;
                    append = false;
                }
                _ => command_args.push(arg),
            }
        }

        commands.push((command, command_args, input, output, append));
        commands
    }

    /// Вспомогательная функция для получения следующего аргумента
    fn next_arg<'a>(&self, args: &'a [&str]) -> &'a str {
        args.iter().next().unwrap_or(&"")
    }
}
