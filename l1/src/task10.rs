use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn task(n: usize) {
    let nums: Vec<usize> = (1..=n).collect();
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        for num in rx1 {
            let square = num * num;
            println!("Отправка квадрата: {square}");
            tx2.send(square).unwrap();
        }
    });

    thread::spawn(move || {
        for square in rx2 {
            println!("Получен квадрат: {square}");
        }
    });

    for num in nums {
        println!("Отправка числа: {num}");
        tx1.send(num).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}


fn main() {
    task(10)
}
