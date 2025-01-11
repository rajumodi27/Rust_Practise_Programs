trait Speak {
    fn speak(&self) {
        println!("Readmore");
    }
}

struct S1 {
    name: String, // Use `String` for owned data
}

impl Speak for S1 {
    // Optionally override the default `speak` method
    fn speak(&self) {
        println!("My name is {}", self.name);
    }
}

fn main() {
    let p = S1 {
        name: String::from("raju"), // Initialize with an owned `String`
    };

    p.speak(); // Outputs: My name is raju
}
