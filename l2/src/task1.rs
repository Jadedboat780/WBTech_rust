use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
enum Flags {
    W,
    L,
    C,
}

#[derive(Debug)]
struct Args<'a> {
    flag: Flags,
    file_path: &'a str,
}

impl<'a> Args<'a> {
    /// Конструктор
    fn new(flag: Option<&str>, file_path: &'a str) -> Self {
        let flag = match flag {
            Some("-c") => Flags::C,
            Some("-l") => Flags::L,
            Some("-w") => Flags::W,
            Some(wrong_option) => {
                eprintln!("Неизвестная опция: {}", wrong_option);
                std::process::exit(1); // Завершение программы с кодом ошибки
            }
            None => Flags::W,
        };

        Args { flag, file_path }
    }

    /// Сопоставление аргументов командной строки
    fn match_arguments(&self) {
        let result = match self.flag {
            Flags::C => self.count_chars(),
            Flags::L => self.count_lines(),
            Flags::W => self.count_words(),
        };

        match result {
            Ok(count) => println!("{}", count),
            Err(_) => eprintln!("Ошибка: файл не найден"),
        }
    }

    /// Считает количество символов в файле
    fn count_chars(&self) -> io::Result<usize> {
        let mut file = File::open(self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content.chars().count())
    }

    /// Считает количество строк в файле
    fn count_lines(&self) -> io::Result<usize> {
        let file = File::open(self.file_path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines().count())
    }

    /// Считает количество слов в файле
    fn count_words(&self) -> io::Result<usize> {
        let file = File::open(self.file_path)?;
        let reader = BufReader::new(file);
        let mut word_count = 0;
        for line in reader.lines() {
            let line = line?;
            word_count += line.split_whitespace().count();
        }
        Ok(word_count)
    }
}

// Для запуска используйте команду вида:
// cargo run --example l2-task1 wc files/song.txt
// cargo run --example l2-task1 wc -l files/song.txt
fn main() {
    // считываем аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // если аргументов командной строки недостаточно или они переданны не коректно, то программа завершается
    if args.len() < 3 || args[1] != "wc" {
        eprintln!("Используйте команду вида: wc [-c|-l|-w] <filename>");
        std::process::exit(1); // Завершение программы с кодом ошибки
    }

    // сопоставление аргументов
    let args = if args.len() == 3 {
        Args::new(None, &args[2])
    } else {
        Args::new(Some(&args[2]), &args[3])
    };

    args.match_arguments();
}
