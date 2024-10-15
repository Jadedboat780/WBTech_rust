# Задание
Создайте микросервисную систему управления задачами на Rust с использованием фреймворка Axum, 
которая взаимодействует с базой данных PostgreSQL и очередью сообщений Redis. 
Проект должен быть упакован в Docker-контейнеры, а для управления зависимостями и оркестрацией используйте Docker Compose. 
Помимо стандартных CRUD операций для задач, реализуйте функциональность уведомления пользователей о новых и завершённых задачах через Redis.

## Требования
* Используйте фреймворк Axum для создания веб-сервиса
* Используйте PostgreSQL для хранения данных о задачах
* Используйте Redis для очередей сообщений и уведомлений пользователей
* Упакуйте все компоненты в Docker-контейнеры
* Настройте Docker Compose для оркестрации контейнеров
* Реализуйте взаимодействие между микросервисами через HTTP API и очередь сообщений
* Реализуйте механизм уведомлений пользователей о новых и завершённых задачах через Redis