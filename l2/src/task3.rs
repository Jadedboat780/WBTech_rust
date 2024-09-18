use clap::Parser;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

/// Утилита для сортировки строк в файле
#[derive(Parser)]
#[clap(author = "Tokin Nikita", version = "1.0", about = "Утилиты сортировки содержимого файлов")]
struct Sort {
    /// Input файл
    input: PathBuf,

    /// Output файл
    output: PathBuf,

    /// Указание колонки для сортировки
    #[clap(short = 'k', long, default_value = "1")]
    column: usize,

    /// Сортировать по числовому значению
    #[clap(short = 'n', long)]
    numeric: bool,

    /// Сортировать в обратном порядке
    #[clap(short = 'r', long)]
    reverse: bool,

    /// Удалить дубликаты строк
    #[clap(short = 'u', long)]
    unique: bool,

    /// Сортировать по названию месяца
    #[clap(short = 'M', long)]
    month_sort: bool,

    /// Игнорировать хвостовые пробелы
    #[clap(short = 'b', long)]
    ignore_trailing_spaces: bool,

    /// Проверить, отсортированы ли данные
    #[clap(short = 'c', long)]
    check_sorted: bool,

    /// Сортировать по числовому значению с учетом суффиксов
    #[clap(short = 'H', long)]
    human_numeric: bool,
}

// cargo run --example l2-task3 -- --help
// cargo run --example l2-task3 -- files/input.txt files/output.txt  -k 2 -n -c
// cargo run --example l2-task3 -- files/input.txt files/output.txt -k 2 -n -r -u
fn main() {
    let args = Sort::parse();

    // Чтение input файла
    let content = fs::read_to_string(&args.input).expect("Не удалось прочитать input файл");

    // Разделение строк файла
    let mut lines: Vec<&str> = content.lines().collect();

    // Удаление дубликатов (если опция -u включена)
    if args.unique {
        let mut seen = HashSet::new();
        lines.retain(|line| seen.insert(line.to_string()));
    }

    // Проверка, отсортирован ли input файл (если опция -c включена)
    if args.check_sorted {
        if is_sorted(&lines, &args) {
            println!("Файл уже отсортирован.");
        } else {
            println!("Файл не отсортирован.");
        }
        return;
    }

    // Сортировка строк
    lines.sort_by(|a, b| {
        let a_key = get_column(a, args.column, args.ignore_trailing_spaces);
        let b_key = get_column(b, args.column, args.ignore_trailing_spaces);

        let cmp_result = if args.month_sort {
            compare_as_month(&a_key, &b_key)
        } else if args.human_numeric {
            compare_as_human_numbers(&a_key, &b_key)
        } else if args.numeric {
            compare_as_numbers(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        // Сортировка в обратном порядке (если опция -r включена)
        if args.reverse {
            cmp_result.reverse()
        } else {
            cmp_result
        }
    });

    // Запись результата в output файл
    fs::write(&args.output, lines.join("\n")).expect("Не удалось записать в output файл");
}

/// Проверка, отсортированы ли строки
fn is_sorted(lines: &[&str], args: &Sort) -> bool {
    for i in 1..lines.len() {
        let a_key = get_column(lines[i - 1], args.column, args.ignore_trailing_spaces);
        let b_key = get_column(lines[i], args.column, args.ignore_trailing_spaces);

        let cmp_result = if args.month_sort {
            compare_as_month(&a_key, &b_key)
        } else if args.human_numeric {
            compare_as_human_numbers(&a_key, &b_key)
        } else if args.numeric {
            compare_as_numbers(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        if cmp_result != Ordering::Less && !args.reverse {
            return false;
        } else if cmp_result != Ordering::Greater && args.reverse {
            return false;
        }
    }
    true
}

/// Извлечение колонки для сортировки
fn get_column(line: &str, column: usize, ignore_trailing_spaces: bool) -> String {
    let line = if ignore_trailing_spaces {
        line.trim_end()
    } else {
        line
    };

    line.split_whitespace()
        .nth(column - 1)
        .unwrap_or("")
        .to_string()
}

/// Сравнение строк как чисел
fn compare_as_numbers(a: &str, b: &str) -> Ordering {
    let a_num = a.parse::<i32>();
    let b_num = b.parse::<i32>();

    match (a_num, b_num) {
        (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(Ordering::Equal),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => a.cmp(b),
    }
}

/// Сравнение строк как месяцев
fn compare_as_month(a: &str, b: &str) -> Ordering {
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    let a_idx = months.iter().position(|&m| m == a_lower);
    let b_idx = months.iter().position(|&m| m == b_lower);

    match (a_idx, b_idx) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

/// Сравнение строк как "человеческие" числа (например "10K", "5M")
fn compare_as_human_numbers(a: &str, b: &str) -> Ordering {
    let re = Regex::new(r"^(\d+)([KMGTP]?)$").unwrap();

    let parse_human_number = |s: &str| -> Option<i32> {
        if let Some(captures) = re.captures(s) {
            let num = captures[1].parse::<i32>().ok()?;
            let suffix = &captures[2];
            let multiplier = match suffix {
                "K" => 1_000,
                "M" => 1_000_000,
                "G" => 1_000_000_000,
                _ => 1,
            };
            return Some(num * multiplier);
        }
        None
    };

    let a_num = parse_human_number(a).unwrap_or(0);
    let b_num = parse_human_number(b).unwrap_or(0);

    a_num.partial_cmp(&b_num).unwrap_or(Ordering::Equal)
}
