use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};

async fn worker(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let mut rx = rx.lock().await;
        if let Some(value) = rx.recv().await {
            println!("Воркер №{} получил: {}", id, value);
        } else {
            break;
        }
    }
}

async fn task(num_workers: usize) {
    let (tx, rx) = mpsc::channel(100);
    let rx = Arc::new(Mutex::new(rx));
    let mut counter = 0;

    for id in 1..=num_workers {
        let worker_rx = rx.clone();
        tokio::spawn(worker(id, worker_rx));
    }

    loop {
        if tx.send(counter).await.is_err() {
            break;
        }
        println!("Отправлено значение: {}", counter);
        counter += 1;
        time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let num_workers = 5;
    task(num_workers).await;


    // Завершает программу по сигналу
    tokio::signal::ctrl_c().await.unwrap();
    println!("Завершение программы");
}

