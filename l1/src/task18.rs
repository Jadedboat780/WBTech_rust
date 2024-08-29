fn task(string: String) -> String {
    string.chars().rev().collect()
}

fn main() {
    let result = task("главрыба".to_string());
    println!("{result}")
}