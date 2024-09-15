use clap::Parser;
use reqwest::Url;
use scraper::{Html, Selector};
use tokio::fs as tokio_fs;
use std::path::Path;
use std::collections::{HashSet, VecDeque};

/// Утилита для скачивания сайта целиком
#[derive(Parser, Debug)]
#[clap(author = "Tokin Nikita", version = "1.0")]
struct WgetArgs {
    /// URL для скачивания
    url: String,

    /// Директория для сохранения
    #[clap(short = 'd', long)]
    directory: Option<String>,
}

/// Загрузка контента по URL
async fn download_page(url: &Url) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url.clone()).await?;
    let body = response.text().await?;
    Ok(body)
}

/// Сохранение веб-страницы в файл
async fn save_page(path: &Path, content: &str) {
    tokio_fs::create_dir_all(path.parent().unwrap()).await.unwrap();
    tokio_fs::write(path, content).await.unwrap();
}

/// Извлечение ссылок из HTML-документа
fn extract_links(base_url: &Url, html: &str) -> HashSet<Url> {
    let mut links = HashSet::new();
    let document = Html::parse_document(html);

    // Селекторы для извлечения всех ссылок
    let selector = Selector::parse("a, link, img, script").unwrap();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href").or(element.value().attr("src")) {
            if let Ok(abs_url) = base_url.join(link) {
                links.insert(abs_url);
            }
        }
    }

    links
}

/// Функция для загрузки и сохранения страницы, а также всех ресурсов
async fn download_site(url: &Url, directory: &Path, visited: &mut HashSet<Url>) {
    let mut queue = VecDeque::new();
    queue.push_back(url.clone());

    while let Some(current_url) = queue.pop_front() {
        if visited.contains(&current_url) {
            continue;
        }
        visited.insert(current_url.clone());

        // Загрузка страницы
        if let Ok(content) = download_page(&current_url).await {
            let path = directory.join(current_url.path().trim_start_matches('/'));
            save_page(&path, &content).await;

            // Извлечение всех ссылок
            let links = extract_links(&current_url, &content);

            // Добавляем новые ссылки в очередь для обработки
            for link in links {
                if !visited.contains(&link) {
                    queue.push_back(link);
                }
            }
        }
    }
}

// cargo run --example l2-task9 -- --help
// cargo run --example l2-task9 https://tech.wildberries.ru/cabinet/courses/rust -d dir
#[tokio::main]
async fn main() {
    // получение аргументов командной строки
    let cli = WgetArgs::parse();

    // Обработка URL
    let url = Url::parse(&cli.url).expect("Invalid URL");

    // Создание директории для сохранения файлов
    let save_directory = cli.directory.unwrap_or_else(|| "download".to_string());
    let save_path = Path::new(&save_directory);

    // Множество для отслеживания посещенных URL
    let mut visited = HashSet::new();

    // Загрузка сайта
    download_site(&url, &save_path, &mut visited).await;
}
