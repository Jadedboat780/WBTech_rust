use flume::{unbounded, Receiver};
use tokio::time::{self, Duration};

async fn worker(id: usize, rx: Receiver<i32>) {
    loop {
        tokio::select! {
            result = rx.recv_async() => {
                match result {
                    Ok(value) => {
                        println!("Воркер {} получил: {}", id, value);
                    },
                    Err(err) => {
                        println!("Ошибка при получении данных: {}", err);
                        break;
                    },
                }
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Воркер {} завершен по сигналу", id);
                break;
            },
        }
    }
}

async fn task(num_workers: usize) {
    let (tx, rx) = unbounded();

    for id in 1..=num_workers {
        let worker_rx = rx.clone();
        tokio::spawn(worker(id, worker_rx));
    }

    let mut counter = 0;

    loop {
        tx.send(counter).unwrap();
        counter += 1;
        time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let num_workers = 4;
    tokio::spawn(async move {
        task(num_workers).await;
    });

    tokio::signal::ctrl_c().await.unwrap();     // Ожидание сигнала Ctrl+C
    println!("Получен сигнал Ctrl+C, завершение работы...");

    time::sleep(Duration::from_secs(1)).await; // ожидание завершения воркеров
    println!("Работа завершена.");
}

// Обоснование способа завершения всех воркеров:
//
// После нажатия Ctrl+C воркеры получают сигнал о завершении программы и прекращают свою работу.
// Чтобы все воркеры успели завершиться, основная программа ждёт небольшой промежуток времени,
// что обеспечивает аккуратное завершение их работы и освобождение ресурсов.