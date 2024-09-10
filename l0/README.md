## Задача
Необходимо разработать демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе. 
[Модель данных в формате JSON](https://drive.google.com/file/d/1rrA7SJUoaGQwDriyY56MAeLT0J_OQkZF/view?usp=sharing) прилагается к заданию.
Приложить к заданию запросы для тестирования и проверки.

## Для запуска проекта выполните команду
```shell
RUST_LOG=info cargo run --release
```

## Эндпоинты:
```shell
GET http://127.0.0.1:3000/ # приветственное сообщение 
GET http://127.0.0.1:3000/orders/WBILMTESTTRACK # получение заказа по трек-номеру
POST http://127.0.0.1:3000/orders # добавление заказа
```
Примеры тестовых запросов есть в папке [http](http)

### Бенчмарки
```shell
tokin_nikita@TokinNikita:~$ wrk -t12 -c400 -d30s http://127.0.0.1:3000/
Running 30s test @ http://127.0.0.1:3000/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.93ms    3.08ms 131.56ms   98.26%
    Req/Sec    18.83k     2.78k   76.89k    84.43%
  6757114 requests in 30.05s, 1.46GB read
Requests/sec: 224826.50
Transfer/sec:     49.74MB
```

С кешированием ручки orders:
```shell
tokin_nikita@TokinNikita:~$ wrk -t12 -c400 -d30s http://127.0.0.1:3000/orders/WBILMTESTTRACK
Running 30s test @ http://127.0.0.1:3000/orders/WBILMTESTTRACK
12 threads and 400 connections
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency   102.04ms  133.02ms   1.09s    86.32%
Req/Sec   642.24    662.88     5.58k    86.90%
191489 requests in 30.09s, 232.11MB read
Requests/sec:   6364.10
Transfer/sec:      7.71MB
```

Без кеширования ручки orders:
```shell
tokin_nikita@TokinNikita:~$ wrk -t12 -c400 -d30s http://127.0.0.1:3000/orders/WBILMTESTTRACK
Running 30s test @ http://127.0.0.1:3000/orders/WBILMTESTTRACK
12 threads and 400 connections
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency   211.88ms   25.40ms 397.03ms   76.32%
Req/Sec   157.55     77.23     0.97k    60.23%
55978 requests in 30.07s, 67.85MB read
Requests/sec:   1861.77
Transfer/sec:      2.26MB
```