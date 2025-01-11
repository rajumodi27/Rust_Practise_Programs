macro_rules! repeat_hello {
    ($text:expr, $count:expr) => {
        for i in 0..$count {
            println!("{} #{}", $text, i); 
        }
    };
}



fn main() {
    repeat_hello!("hello", 5); 
}
