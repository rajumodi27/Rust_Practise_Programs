// fn main() {
//     let x = 1;
//     match x {
//         5 => println!("five"),
//         6 => println!("six"),
//         7 => println!("seven"),
//         _=>println!("asd"),
//     }
// }



fn main() {
    let age = 25;

    match age {
        n if n < 18 => println!("Minor"),
        n if n >= 18 && n <= 65 => println!("Adult"),
        _ => println!("Senior"),
    }
}
