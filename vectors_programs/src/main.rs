fn main() {

    //Example1
    // let mut v:Vec<i32>=Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    // v.push(5);
    // println!("{:?}",v);

    //Example2
    // let v=vec![1,2,3];
    // println!("{:?}",v);


    let mut v:Vec<&str>=Vec::new();
    v.push("a");
    v.push("b");
    v.push("c");
    println!("{:?}",v);

    myfun(&mut v);

    println!("{:?}",v);
}

fn myfun(v1:&mut Vec<&str>){
    v1.push("d");
    println!("{:?}",v1);
}