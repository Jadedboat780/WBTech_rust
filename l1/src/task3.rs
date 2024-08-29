use std::sync::mpsc;
use std::thread;

fn task(n: usize) {
    let nums: Vec<usize> = (1..=n).collect();
    let (tx, rx) = mpsc::channel();
    let mut sum = 0;

    for num in nums {
        let tx = tx.clone();
        thread::spawn(move || {
            let square = num * num;
            tx.send(square).unwrap()
        });
    }

    drop(tx);

    while let Ok(received) = rx.recv() {
        sum += received
    }

    println!("Сумма квадратов равна {sum}")
}

fn main() {
    task(5)
}
