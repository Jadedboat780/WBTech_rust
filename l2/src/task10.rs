use clap::Parser;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::io::{self as aio, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

/// Telnet-клиент
#[derive(Parser)]
#[clap(author = "Tokin Nikita", version = "1.0", about = "Simple Telnet client")]
struct TelnetArgs {
    /// Хост для подключения
    host: String,

    /// Порт для подключения
    port: u16,

    /// Таймаут подключения (по умолчанию 10 секунд)
    #[clap(short = 't', long, default_value="10")]
    timeout: u64,
}

/// Получение адреса
fn get_addr(host: String, port: u16) -> Option<SocketAddr> {
    let address = format!("{}:{}", host, port);
    match address.to_socket_addrs() {
        Ok(mut addrs) => Some(addrs.next().expect("Не удалось разрешить адрес")),
        Err(_) => None
    }
}

// cargo run --example l2-task10 -- --help
//
// cargo run --example l2-task10 --  --timeout=3 mysite.ru 8080
//
// cargo run --example l2-task10 -- beget.com 80
// > GET / HTTP/1.1
#[tokio::main]
async fn main() {
    let args = TelnetArgs::parse();

    let timeout_duration = Duration::from_secs(args.timeout);
    let addr = get_addr(args.host, args.port).expect("Неверный адрес");
    let token = CancellationToken::new();

    let stream = match timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(s)) => {
            println!("Успешное подключение к {}", addr);
            s
        }
        Ok(Err(e)) => {
            eprintln!("Ошибка при подключении: {}", e);
            return;
        }
        Err(_) => {
            eprintln!("Таймаут подключения");
            return;
        }
    };

    let (mut reader, mut writer) = stream.into_split();
    let task_token = token.clone();
    // Создание задачи для обработки STDIN и записи в сокет
    let stdin_to_socket = tokio::spawn(async move {
        let mut stdin = aio::stdin();
        let mut buffer = [0; 1024];
        loop {
            let n = match stdin.read(&mut buffer).await {
                Ok(n) if n == 0 => {
                    // Конец ввода (Ctrl+D)
                    println!("Закрытие подключения.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Ошибка чтения из STDIN: {}", e);
                    break;
                }
            };

            if let Err(e) = writer.write_all(&buffer[0..n]).await {
                eprintln!("Ошибка записи в сокет: {}", e);
                break;
            }

            if task_token.is_cancelled() {
                break;
            }
        }
    });

    let task_token = token.clone();
    // Создание задачи для обработки данных из сокета и записи в STDOUT
    let socket_to_stdout = tokio::spawn(async move {
        let mut stdout = aio::stdout();
        let mut buffer = [0; 1024];
        loop {
            let n = match reader.read(&mut buffer).await {
                Ok(n) if n == 0 => {
                    println!("Подключение закрыто сервером.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Ошибка чтения из сокета: {}", e);
                    break;
                }
            };

            if let Err(e) = stdout.write_all(&buffer[0..n]).await {
                eprintln!("Ошибка записи в STDOUT: {}", e);
                break;
            }

            if task_token.is_cancelled(){
                break;
            }
        }
    });

    // Ожидание завершения любой из задач (чтение из stdin или сокета)
    tokio::select! {
        _ = stdin_to_socket => {
            token.cancel()
        }
        _ = socket_to_stdout => {
            token.cancel()
        }
    }
}
