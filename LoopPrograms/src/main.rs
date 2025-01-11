

//Loop

// fn main() {
//     let mut counter = 0;
    
//     loop {
//         counter += 1;
//         println!("Counter: {}", counter);

//         if counter == 5 {
//             break;  
//         }
//     }
// }


//____________________________

// fn main(){

// println!("hello {}","world");


// }








// _________________________

//while loop

// fn main()
// {
    
//     let mut count:i8=0;
//     while count<10
//     {
//         println!("Count :{}",count);
//         count+=1;
//     }
// }




// _________________________
//for loop

// fn main(){

//     for i in 0..5
//     {
//         println!("{}",i);
//     }
// }



// _________________________
// for loop with  iter

// fn main(){

//     let mut x=[1,2,3,4,5];
//     for i in x.iter_mut()
//     {
            // *i+=1;        
    
//     }
//     println!("{:?}",x);


// }


//Vector
// _______________________

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5]; // Create a vector with initial values
//     println!("{:?}", numbers);         // Output: [1, 2, 3, 4, 5]
// }



// fn main() {
//     let mut x = vec![1, 2, 3]; // Growable vector
//     x.push(4);                // Add 4 to the vector
//     println!("{:?}", x);      // Output: [1, 2, 3, 4]
// }


// triangle pattern
// __________________________

// fn main() {
//     for i in 1..=5 
//     {  

//         for j in 0..i 
//         {  
//             print!("*");  
//         }

//         println!("\n");  
//     }
// }



// fn main() {
//     for i in (1..=5).rev() 
//     {  

//         for j in 0..i 
//         {  
//             print!("*");  
//         }

//         println!("\n");  
//     }
// }




// fullTriangle
// fn main() {
//     for i in 1..=5 
//     {  

//         for _ in (i..5).rev()
//         {
//                 print!(" ");
//             }


//         for _ in 0..i 
//         {  
//             print!("*");  
//         }


//         for _ in (0..i-1).rev() 
//         {  
//             print!("*");  
//         }



//         println!("\n");  
//     }
// }




// ______________

// full diamond

// fn main() {
    // for i in 1..=5 {
    //     for _ in i..5 {
    //         print!(" ");
    //     }

    //     for _ in 1..=2*i - 1 {
    //         print!("*");
    //     }

    //     println!(); 
    // }




    // for i in (1..5).rev() {
    //     for _ in i..5 {
    //         print!(" ");
    //     }

    //     for _ in 1..=2*i - 1 {
    //         print!("*");
    //     }

    //     println!(); 
    // }
// }


fn main()
{

    let x =[1,2,3,4];
    for i in x{
        println!("{}",i);
    }


}






