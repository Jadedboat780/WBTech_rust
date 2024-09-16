use clap::Parser;
use std::io::{self, BufRead};

/// Утилита cut: выбирает определенные поля из строк
#[derive(Parser, Debug)]
#[clap(author = "Tokin Nikita", version = "1.0", about = "An analog of cut command")]
struct CutArgs {
    /// Поля для выбора (например, 1,2,3)
    #[clap(short = 'f', long)]
    fields: String,

    /// Разделитель (по умолчанию TAB)
    #[clap(short = 'd', long, default_value = "\t")]
    delimiter: String,

    /// Только строки с разделителем
    #[clap(short = 's', long)]
    separated: bool,
}

// cargo run --example l2-task6 -- --help
// echo -e "one\ttwo\tthree" | cargo run --example l2-task6 -- -f 1,3
// echo "one,two,three" | cargo run --example l2-task6 -- -f 1,3 -d ","
// echo -e "one two three\none\ttwo\tthree" | cargo run --example l2-task6 -- -f 2 -s
fn main() {
    // Получение аргументов командной строки
    let args = CutArgs::parse();

    // Парсинг поля
    let fields: Vec<usize> = args
        .fields
        .split(',')
        .filter_map(|f| f.parse::<usize>().ok())
        .collect();

    // Чтение строк из STDIN
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|line| line.unwrap()) {
        // Проверка, содержит ли строка разделитель
        if args.separated && !line.contains(&args.delimiter) {
            continue;
        }

        // Разбиение строки по разделителю
        let columns: Vec<&str> = line.split(&args.delimiter).collect();

        // Выборка запрошенных колонок
        let selected_fields: Vec<&str> = fields
            .iter()
            .filter_map(|field| columns.get(field - 1))
            .cloned()
            .collect();

        // Вывод результата
        if !selected_fields.is_empty() {
            println!("{}", selected_fields.join(&args.delimiter));
        }
    }
}
