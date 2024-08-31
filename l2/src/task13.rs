/// Кортежная структура из одного элемента (числа)
struct Example(i32);

/// Реализация диструктора
impl Drop for Example {
    fn drop(&mut self) {
        // при уничтожении объекта, будет выводиться его значение
        println!("{}", self.0);
    }
}

/// Кортежная структура из одного элемента (другая кортежная структура)
struct ExampleWrap(Example);

/// Реализация диструктора
impl Drop for ExampleWrap {
    fn drop(&mut self) {
        let e = std::mem::replace(&mut self.0, Example(0));
        println!("wrap {}", e.0);
    }
}

/// Точка входа в программу
fn main() {
    // Объект создаётся и сразу уничтожается, так как нет переменной для хранения значения
    Example(1);

    // Объект создаётся и присваивается неиспользуемой переменной.
    // Объект будет уничтожен при выходе из области видимости функции main
    let _e2 = Example(2);

    // Объект создаётся и присваивается неиспользуемой переменной.
    // Объект будет уничтожен при выходе из области видимости функции main
    let _e3 = Example(3);

    // Объект создаётся и сразу уничтожается, так как нет переменной для хранения значения
    let _ = Example(4);

    // Создание мутабельной переменной
    let mut _e5;

    // Присвоение переменной значения
    _e5 = Some(Example(5));

    // Присвоение переменной нового значения.
    // При изменении значения переменной вызывается диструктор
    _e5 = None;

    // Создание переменной и присвоение ей значения
    let e6 = Example(6);

    // Явный вызов диструктора
    drop(e6);

    // Создание переменной и присвоение ей значения
    let e7 = Example(7);

    // Отмена вызова диструктора для значения при выходе из области видимости функции
    std::mem::forget(e7);

    // Объект создаётся и сразу уничтожается, так как нет переменной для хранения значения
    // Сначала будет уничтожена обёртка ExampleWrap, а после само значение Example
    ExampleWrap(Example(8));
}

// Уничтожение объектов диструктором при выходе из области видимости функции
// происходит в обратном порядке их созданию