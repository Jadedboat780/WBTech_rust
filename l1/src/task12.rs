fn task<T: PartialEq + Clone>(set1: &[T], set2: &[T]) -> Vec<T> {
    let mut result = Vec::new();

    for item in set1 {
        if set2.contains(item) && !result.contains(item) {
            result.push(item.clone());
        }
    }

    result
}

fn main() {
    let set1 = vec![1, 2, 3, 4, 5];
    let set2 = vec![4, 5, 6, 7, 8];

    let result = task(&set1, &set2);
    println!("Пересечение: {:?}", result);
}
