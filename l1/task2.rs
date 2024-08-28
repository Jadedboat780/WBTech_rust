use std::thread;

fn task(n: usize) {
    let mut handles = Vec::new();
    let nums: Vec<usize> = (1..=n).collect();

    for num in nums {
        let handle = thread::spawn(move || {
            let square = num * num;
            println!("Квадрат от {} равен {}", num, square);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    task(10)
}
