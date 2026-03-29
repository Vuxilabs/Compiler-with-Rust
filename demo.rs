struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn print_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}