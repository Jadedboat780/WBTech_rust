# Задание
Необходимо разработать упрощенную мини-социальную сеть, где пользователи могут регистрироваться, авторизовываться, 
публиковать сообщения и ставить лайки.

Проект должен включать следующие функции:
* Регистрация и авторизация пользователей: пользователи могут создавать учетные записи и входить в систему 
* Публикация сообщений: пользователи могут создавать и удалять свои сообщения 
* Лайки: пользователи могут ставить лайки сообщениям

## Требования
* Используйте фреймворк Axum для создания веб-сервиса. 
* Используйте СУБД PostgreSQL и библиотеку tokio-postgres для работы с данными 
* Используйте JWT для аутентификации и авторизации 
* Используйте Tokio и async/await для асинхронного выполнения задач 
* Обеспечьте валидацию данных и обработку ошибок

## Структура API
* POST /register: регистрация нового пользователя 
* POST /login: авторизация пользователя 
* POST /posts: создание нового сообщения 
* GET /posts/{post_id}: получение сообщения 
* DELETE /posts/{post_id}: удаление сообщения 
* POST /posts/{post_id}/likes: лайк сообщения.

## Для запуска проекта выполните:
```shell
cargo run 
```