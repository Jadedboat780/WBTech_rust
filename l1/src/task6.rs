use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_util::sync::CancellationToken;

async fn task(second: Duration) {
    let (tx, mut rx) = mpsc::channel(100);
    let cancellation_token = CancellationToken::new();

    let task1 = {
        let tx = tx.clone();
        let cancellation_token = cancellation_token.clone();

        tokio::spawn(async move {
            let mut counter = 0;
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }
                if tx.send(counter).await.is_err() {
                    break;
                }
                counter += 1;
                time::sleep(Duration::from_millis(500)).await;
            }
        })
    };

    let task2 = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Получено: {}", value);
        }
    });


    time::sleep(second).await;
    cancellation_token.cancel();
    drop(tx);

    task1.await.unwrap();
    task2.await.unwrap();
}

#[tokio::main]
async fn main() {
    task(Duration::from_secs(5)).await;
}
