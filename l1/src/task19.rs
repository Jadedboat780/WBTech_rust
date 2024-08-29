fn task(text: &str) -> String {
    let reversed_words: Vec<&str> = text
        .split_whitespace()
        .rev()
        .collect();

    reversed_words.join(" ")
}

fn main() {
    let result = task("snow dog sun");
    println!("{result}")
}