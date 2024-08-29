use std::any::{Any, TypeId};

/// Выводит в stdout тип передаваемой переменной
fn task1<T: Any>(_value: T) {
    println!("Тип переменной: {}", std::any::type_name::<T>())
}

/// В зависимости от переданного типа производит определённые действия
fn task2<T: Any>(_value: T) {
    let type_id = TypeId::of::<T>();

    if type_id == TypeId::of::<i32>() {
        println!("Тип переменной: i32");
    } else if type_id == TypeId::of::<f64>() {
        println!("Тип переменной: f64");
    } else if type_id == TypeId::of::<String>() {
        println!("Тип переменной: String");
    } else if type_id == TypeId::of::<bool>() {
        println!("Тип переменной: bool");
    } else {
        println!("Неизвестный тип");
    }
}

fn main() {
    task1(10u8);
    task1("Hello");
    task1(vec![1, 2, 3]);

    task2(true);
    task2("Hello".to_string());
    task2(vec![1, 2, 3]);
}