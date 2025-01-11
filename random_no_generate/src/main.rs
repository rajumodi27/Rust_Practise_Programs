use rand::prelude::*;
use std::io;

fn main() {
    let fruit = ["apple", "banana", "mango", "pineapple"];
    let mut rng = thread_rng();
    let rand_no = rng.gen_range(0..fruit.len());

    println!("Guess the fruit!");

    let mut guess_fruit_name = String::new();
    loop {
        guess_fruit_name.clear();
        io::stdin()
            .read_line(&mut guess_fruit_name)
            .expect("failed to read");

        if guess_fruit_name.trim() == fruit[rand_no] {
            println!("Found it!");
            break;
        } else {
            println!("Retry.");
        }
    }
}
