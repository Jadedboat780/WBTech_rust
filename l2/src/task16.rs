/// Принимает срез целых чисел и возвращает приёмник,
/// который содержит значения из переданного среза
fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    // Создание канала для передачи 32 битных знаковых чисел.
    // tx - передатчик
    // rx - приёмник
    let (tx, rx) = std::sync::mpsc::channel();

    // Создание потока
    let handle = std::thread::spawn({
        // Клонирование переданного в функцию среза
        let vs = vs.to_owned();

        move || {
            // Итерация по элементам вектора
            for v in vs {
                // Отправка значения
                tx.send(v).unwrap();
                // Остановка работы потока на 1 секунду
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            // Уничтожение передатчика, чтобы больше нельзя было отправлять данные в приёмник
            drop(tx);
        }
    });
    // Ожидание завершения потока
    handle.join().unwrap();

    // возврат приёмника из функции
    rx
}

/// Объединяет два канала в один
fn merge(a: std::sync::mpsc::Receiver<i32>, b: std::sync::mpsc::Receiver<i32>) -> std::sync::mpsc::Receiver<i32> {
    // Создание канала для передачи 32 битных знаковых чисел.
    // tx - передатчик
    // rx - приёмник
    let (tx, rx) = std::sync::mpsc::channel();

    // флаги для отслеживания завершения получения значений из обоих каналов
    let mut a_done = false;
    let mut b_done = false;

    loop {
        // Попытка получить значение из приёмника
        match a.try_recv() {
            // если в канале ещё есть значения
            Ok(i) => {
                // отправка значения в новый приёмник
                tx.send(i).unwrap();
            }

            // если канал пуст или закрыт
            Err(_) => {
                // установка значения флагу, означающая, что все данные из канала получены
                a_done = true;
            }
        }

        // Попытка получить значение из приёмника
        match b.try_recv() {
            // если в канале ещё есть значения
            Ok(i) => {
                // отправка значения в новый приёмник
                tx.send(i).unwrap();
            }

            // если канал пуст или закрыт
            Err(_) => {
                // установка значения флагу, означающая, что все данные из канала получены
                b_done = true;
            }
        }

        // Если оба канала закрыты, то цикл заканчивается
        if a_done && b_done {
            break;
        }
    }

    // Возвращение объединённого канала
    rx
}

/// Точка входа в программу
fn main() {
    // Создание двух приёмников
    let a = as_chan(&vec![1, 3, 5, 7]);
    let b = as_chan(&vec![2, 4, 6, 8]);

    // Объединение приёмников
    let c = merge(a, b);

    // Итерация по значениям, переданных в канал
    for v in c.iter() {
        // Вывод значения в консоль
        println!("{v:?}");
    }
}