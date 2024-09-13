use clap::{value_parser, Arg, ArgMatches, Command};
use rayon::{
    prelude::{ParallelIterator, ParallelString},
    ThreadPoolBuilder,
};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Подсчитывает количество символов в строке
fn count_letters(text: &str) -> HashMap<char, usize> {
    let freq_map = Arc::new(Mutex::new(HashMap::new()));

    text.par_chars()
        // фильтруем только латинские символы
        .filter(|c| c.is_ascii_alphabetic())
        // приводим к нижнему регистру
        .map(|c| c.to_ascii_lowercase())
        .for_each_with(freq_map.clone(), |map, c| {
            // подсчёт символов
            map
                .lock()
                .unwrap()
                .entry(c)
                .or_insert(0)
                .add_assign(1)
        });

    let map = Arc::try_unwrap(freq_map).unwrap().into_inner().unwrap();
    map
}

struct CMD(ArgMatches, Option<(String, usize)>);

impl CMD {
    fn new() -> Self {
        let am = Command::new("Letter Frequency")
            .version("1.0")
            .author("Tokin Nikita")
            .about("Counts the frequency of letters in a text file")
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .help("The input file to analyze")
                    .required(true),
            )
            .arg(
                Arg::new("threads")
                    .short('t')
                    .long("threads")
                    .help("Number of threads to use")
                    .default_value("1")
                    .value_parser(value_parser!(usize)),
            )
            .get_matches();

        CMD(am, None)
    }

    fn match_args(&mut self) {
        let file_path: &String = self.0.get_one("file").expect("");
        let threads: usize = *self.0.get_one("threads").unwrap();

        self.1 = Some((file_path.to_string(), threads))
    }

    fn get_args(self) -> Option<(String, usize)> {
        self.1
    }
}

// cargo run --example l2-task7 -- --help
// cargo run --example l2-task7 -- -f files/song.txt
// cargo run --example l2-task7 -- -f files/song.txt -t 3
fn main() {
    // получение аргументов командной строки
    let mut cmd = CMD::new();
    cmd.match_args();
    let (file_path, threads) = cmd.get_args().unwrap();

    // получение текста из файла и настройка количества потоков
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let _ = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let start_time = Instant::now(); // запуск измерения времени
    let frequencies = count_letters(&content);
    let elapsed_time = start_time.elapsed(); // завершаем измерение времени

    // преобразование результатов в JSON
    let result = json!({
        "elapsed": format!("{:?} ", elapsed_time),
        "result": frequencies
    });

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
