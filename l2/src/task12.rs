/// Точка входа в программу
fn main() {
    // статический массив из знаковых 32 битных чисел
    let a = [76, 77, 78, 79, 80];
    // срез массива с 1 до 3 элемента
    let b = &a[1..4];
    // вызов макроса для вывода значений среза в консоль
    println!("{b:?}");
}