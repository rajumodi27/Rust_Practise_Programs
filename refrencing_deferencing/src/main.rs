fn main() {
        let mut x=5;
        let y=&mut x;
    // println!("{}",x); 
    println!("{}",y); //auto deferencing can also write *y same result instead it should print the address of the x as value but rust auto deference it .
    *y+=1;
    println!("{}",y); 


}
