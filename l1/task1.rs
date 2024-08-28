trait Action {
    fn say(&self);
}

struct Person {
    name: &'static str,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name)
    }
}

fn main() {
    let person = Person { name: "Nikita" };
    person.say()
}
