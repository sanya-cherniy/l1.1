// Описываем трейт
pub trait Action {
    fn say(&self);
}

// Описываем структуру
struct Person {
    name: String,
}

// Описываем функционал
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    // создаем экземпляр структуры и вызываем метод "say"
    let plumber = Person {
        name: String::from("Artur"),
    };
    plumber.say();
}
