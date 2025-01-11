
// fn main() 
// {
//     let name = String::from("Raju");
//     myfunction(name); 
//     // println("{}",name);  //this will throw errro as  owener are transfered
// }

// fn myfunction(val:String){

//     println!("{}",val);
    
// }
    

//Using Clone 
//_____________________________________________________________
// fn myfunction(val:String){

//     println!("{}",val);
    
//     }
    
    
    
//     fn main() {
    
//     let name = String::from("Raju");
//     myfunction(name.clone()); 
//     // println!("{}",name);  //this will throw errro as  owener are transfered
//     println!("{}",name);  //this will throw errro as  owener are transfered
    
//     }
    

    //Also use tuple to fix this 



    //borrow the reference not mutable
//___________________________________________________
    // fn main() 
    // {
    
    //     let mut name0 = String::from("raju");
    //     let name1 = &mut name0; // Mutable reference to `name0`
    
    //     name1.push_str("modi");
    
    //     println!("{}", name1); 
    //     println!("{}", name0); 

    // }


//Data Race
//_________________
// fn main()
// {
//     let name0 = String::from("Raju");
//     let name1=&mut name0;
//     let name2=&mut name0;

//     //no problem here
//     // println!("{}",name0);
//     // println!("{}",name1);
//     // println!("{}",name2);

//     //problem here
//     println!("{} {}",name1,name2);

    
// }




//Ownership rules
//____________________________
//multiple read 
// fn main()
// {

//     let name=String::from("raju");
//     let n1 = &name;
//     let n2 = &name;

//     println!("{} {}",n1,n2);

// }


//______________________________
//multiple write
// fn main()
// {

//     let mut name=String::from("raju");
//     let n1 = &mut name;
//     let n2 = &mut name;

//     println!("{} {}",n1,n2);

// }



//_______________________________
//multiple read/write

fn main()
{

    let mut name=String::from("raju");

   

    let read = &name;
    println!("{}",read);

    let write = &mut name;
    println!("{}",write);

}





