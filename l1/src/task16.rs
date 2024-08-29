use std::cmp::Ordering;

fn task<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let (mut low, mut high) = (0, slice.len());

    while low < high {
        let mid = low + (high - low) / 2;
        match slice[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    None
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 5;
    match task(&nums, &target)  {
        Some(index) =>  println!("Элемент {} найден на позиции {}", target, index),
        None => println!("Элемент {} не найден", target)
    }

    let words = vec!["apple", "banana", "cherry", "date"];
    let target_word = "cherry";
    match task(&words, &target_word)  {
        Some(index) =>  println!("Элемент {} найден на позиции {}", target_word, index),
        None => println!("Элемент {} не найден", target_word)
    }

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 12;
    match task(&nums, &target)  {
        Some(index) =>  println!("Элемент {} найден на позиции {}", target, index),
        None => println!("Элемент {} не найден", target)
    }
}