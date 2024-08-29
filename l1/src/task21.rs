pub trait TextInterface {
    fn process_text(&self, text: i32) -> String;
}

pub trait NumInterface {
    fn process_number(&self, number: i32) -> i32;
}

pub struct System;

pub struct TextToNumAdapter {
    system: System,
}

impl TextToNumAdapter {
    pub fn new(system: System) -> Self {
        TextToNumAdapter { system }
    }
}

impl NumInterface for TextToNumAdapter {
    fn process_number(&self, num: i32) -> i32 {
        let processed_text = self.system.process_text(num);
        processed_text.len() as i32
    }
}


impl TextInterface for System {
    fn process_text(&self, text: i32) -> String {
        format!("{}", text)
    }
}

fn main() {
    let system = System;
    let num = 1234;
    let adapter = TextToNumAdapter::new(system);
    let result = adapter.process_number(num);

    println!("Результат обработки числа {}: {}", num, result);
}
