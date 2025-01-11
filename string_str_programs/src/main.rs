use std::io;

fn main() {

let fname ="Raju";

let mut sname = String::new();
sname.push_str("Modi");

println!("{} {}",fname,sname);

sname.push_str(" Narendra");
println!("{} {}",fname,sname);


io::stdin()
.read_line(&mut sname);
println!("{} {}",fname,sname);




}
