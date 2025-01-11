use std::io;

fn main() 
{
    println!("enter the name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read");
    println!("{}",name);

}
