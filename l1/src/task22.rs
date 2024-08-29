fn task<T: Clone>(vec: Vec<T>, index: usize) -> Option<Vec<T>> {
    if vec.len() < index {
        return None;
    }

    let mut v = Vec::with_capacity(vec.len() - 1);
    v.extend_from_slice(&vec[..index]);
    v.extend_from_slice(&vec[index + 1..]);

    Some(v)
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let index = 5;

    match task(v, index) {
        Some(result) => println!("Результат: {:?}", result),
        None => println!("Индекс за пределами массива."),
    }
}