use std::io::{self, BufRead};

fn task() -> Vec<String> {
    // Для завершения ввода нажмите Ctrl + D
    let mut lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    lines.sort();
    lines.dedup();
    lines
}

fn main() {
    let mut lines = task();

    for line in lines{
        println!("{line}")
    }
}