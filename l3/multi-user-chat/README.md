# Задание
Создайте многопользовательский онлайн-чат, где пользователи могут присоединяться к комнатам, отправлять сообщения 
и получать их в реальном времени. Чат должен поддерживать несколько комнат, и пользователи могут отправлять сообщения 
только в те комнаты, к которым они присоединились. Реализация должна использовать axum для веб-сервера, 
а также различные структуры синхронизации и асинхронные возможности Rust.

## Требования
* Используйте фреймворк Axum для создания веб-сервера
* Используйте Arc, Mutex, RwLock, Box, каналы и tokio для управления состоянием и асинхронной обработки 
* Используйте DashMap для хранения данных о пользователях и комнатах 
* Используйте атомарные переменные (AtomicUsize) для отслеживания количества пользователей

## Структура проекта
Модели данных:
* Пользователь 
* Комната 
* Сообщение

Структура API:
* POST /join: присоединение к комнате.
* POST /leave: покидание комнаты.
* POST /send: отправка сообщения в комнату.
* GET /messages: получение сообщений из комнаты.

## Для запуска проекта выполните:
```shell
cargo run --release
```