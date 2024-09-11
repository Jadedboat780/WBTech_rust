use tokio::sync::broadcast;
use tokio::time::{self, Duration};

async fn worker(id: usize, mut rx: broadcast::Receiver<i32>) {
    loop {
        match rx.recv().await {
            Ok(value) => {
                println!("Воркер №{} получил: {}", id, value);
            }
            Err(err) => {
                println!("У воркера №{} приозошла ошибка: {}", id, err);
            }
        }
    }
}

async fn task(num_workers: usize) {
    let (tx, _) = broadcast::channel(100);
    let mut counter = 0;

    for id in 1..=num_workers {
        let rx = tx.subscribe();
        tokio::spawn(worker(id, rx));
    }

    loop {
        if tx.send(counter).is_err() {
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

    tokio::signal::ctrl_c().await.unwrap();
}
