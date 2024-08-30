use std::thread;
use std::time::{Duration};
use tokio_util::sync::CancellationToken;

/// Остановка канала с использованием thread
fn task1() {
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = thread::spawn(move || {
        while let Ok(val) = rx.recv() {
            println!("Получено: {}", val);
        }
        println!("Задача закрывается");
    });

    for i in 0..=5 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    drop(tx);

    handle.join().unwrap();
}

/// Остановка канала с использованием task
async fn task2() {

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let handle = tokio::spawn(async move {
        while let Some(val) = rx.recv().await {
            println!("Получено: {}", val);
        }
        println!("Задача закрывается");
    });

    for i in 0..=5 {
        tx.send(i).await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    drop(tx);

    handle.await.unwrap();
}

/// Остановка задачи с помощью токена
async fn task3() {
    let token = CancellationToken::new();
    let task_token = token.clone();

    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = task_token.cancelled() => {
                    println!("Задача закрывается");
                    break;
                },
                _ = tokio::time::sleep(Duration::from_millis(500)) => {
                    println!("Задача работает");
                },
            }
        }
    });

    tokio::time::sleep(Duration::from_secs(3)).await;
    token.cancel();
    handle.await.unwrap();
}

#[tokio::main]
async fn main() {
    task3().await;
}