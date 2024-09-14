use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Утилита
#[derive(Parser, Debug)]
#[clap(author = "Tokin Nikita", version = "1.0")]
struct GrepArgs {
    /// Шаблон для поиска
    pattern: String,

    /// Файл для поиска
    file: String,

    /// Печатать +N строк после совпадения
    #[clap(short = 'A', long)]
    after_context: Option<usize>,

    /// Печатать +N строк до совпадения
    #[clap(short = 'B', long)]
    before_context: Option<usize>,

    /// Печатать ±N строк вокруг совпадения
    #[clap(short = 'C', long)]
    context: Option<usize>,

    /// Количество строк с совпадениями
    #[clap(short = 'c', long)]
    count: bool,

    /// Игнорировать регистр
    #[clap(short = 'i', long)]
    ignore_case: bool,

    /// Исключить совпадения (invert match)
    #[clap(short = 'v', long)]
    invert: bool,

    /// Точное совпадение со строкой (не паттерн)
    #[clap(short = 'F', long)]
    fixed: bool,

    /// Напечатать номер строки
    #[clap(short = 'n', long)]
    line_num: bool,
}

// cargo run --example l2-task5 -- --help
// cargo run --example l2-task5 -- war files/song.txt
// cargo run --example l2-task5 -- war files/song.txt -A 1 -B 1 -i -n
// cargo run --example l2-task5 -- war files/song.txt -i -c
fn main() {
    // Получение аргументов командной строки
    let cli = GrepArgs::parse();

    // Открытие файла и создание буфера для чтения
    let file = File::open(&cli.file).expect("Файл не найден");
    let reader = BufReader::new(file);

    // Обработка ключа context (-C), который является комбинацией -A и -B
    let after_context = cli.context.unwrap_or(cli.after_context.unwrap_or(0));
    let before_context = cli.context.unwrap_or(cli.before_context.unwrap_or(0));

    // Счетчика строк
    let mut match_count = 0;

    // Вектор из строк файла
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (line_num, line) in lines.iter().enumerate() {
        // Преобразование строки в нижний регистр (если установлен флаг -i)
        let search_line = if cli.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        // Преобразование шаблона в нижний регистр (если установлен флаг -i)
        let search_pattern = if cli.ignore_case {
            cli.pattern.to_lowercase()
        } else {
            cli.pattern.to_string()
        };

        // Проверка условия совпадения с шаблоном
        let is_match = if cli.fixed {
            search_line == search_pattern
        } else {
            search_line.contains(&search_pattern)
        };

        // Исключение совпадений (если установлен флаг -v)
        let should_print = if cli.invert { !is_match } else { is_match };

        // Логика фильтрации
        if should_print {
            // Если флаг -c установлен - считаем количество совпадений
            if cli.count {
                match_count += 1;
                continue;
            }

            // Печать контекста до (before_context) и после (after_context) совпадения
            let start = if line_num >= before_context {
                line_num - before_context
            } else {
                0
            };

            let end = if line_num + after_context < lines.len() {
                line_num + after_context + 1
            } else {
                lines.len()
            };

            // Печать контекста
            for i in start..end {
                if cli.line_num {
                    println!("{}: {}", i + 1, lines[i]);
                } else {
                    println!("{}", lines[i]);
                }
            }
        }
    }

    // Вывод количества совпадений, если установлен флаг -c
    if cli.count {
        println!("{}", match_count);
    }
}
