use std::collections::HashSet;

fn task(s: &str) -> bool {
    let mut chars_set = HashSet::new();

    for c in s.to_lowercase().chars() {
        if !chars_set.insert(c) {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", task("abcd"));
    println!("{}", task("abCdefAaf"));
    println!("{}", task("aabcd "));
}