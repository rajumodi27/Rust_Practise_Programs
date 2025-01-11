// fn main() {
//     let int_array = [1, 2, 3, 4, 5];
//     let char_array = ['a', 'b', 'c', 'd'];
//     let string_array = ["hello", "world", "Rust"];
//     let float_array = [1.1, 2.2, 3.3, 4.4];
//     let bool_array = [true, false, true];

//     // Directly print arrays
//     println!("Integer Array: {:?}", int_array);
//     println!("Character Array: {:?}", char_array);
//     println!("String Array: {:?}", string_array);
//     println!("Float Array: {:?}", float_array);
//     println!("Boolean Array: {:?}", bool_array);

//     // Print using loops
//     println!("\nPrinting elements using a loop:");
//     for val in int_array {
//         println!("{}", val);
//     }

  
//     // Using indices
//     println!("\nPrinting elements using indices:");
//     for i in 0..string_array.len() {
//         println!("Index {}: {}", i, string_array[i]);
//     }




//Not Covered 
// _______________________________________________________
//   // Using iterators
//   println!("\nPrinting elements using iterators:");
//   char_array.iter().for_each(|ch| println!("{}", ch));


    // // Using enumerate
    // println!("\nPrinting elements with indices using enumerate:");
    // for (index, value) in float_array.iter().enumerate() {
    //     println!("Index: {}, Value: {}", index, value);
    // }
// }



//Count the length of array using function 
//__________________________________________________________

// fn main() {
//     let mut x:[i8;10]=[5;10]; //array declre initialize
//     let c:usize =count_lengths(&mut x); // calling function
//     println!("{}",c);
//     println!("{:?}",x);


// }

// //function to calculate the length of the array
// fn count_lengths(i:&mut [i8;10])->usize
// {   
// i[2]=22;
// i.len()    
// }





//2d - Array 
//_____________________________
// fn main()
// {

// let x:[[i8;3];3]=[
//     [1,2,3],
//     [4,5,6],
//     [7,8,9]
// ];

// println!("{}",x[2][2]);

// }


//mutable reference
//example1
// fn main()
// {
//     let x:i8 =1;
//     mutabl_function(x);
//     println!("{}",x);
// }

// fn mutabl_function(mut val:i8)
// {
    
//     val=2;
//     println!("{}",val);

// }

//example2
// fn main()
// {
//    let arr:[&str;3]=["hello","world","coders"];
//    write_arr(arr); //array drectly pass
//    println!("arr={:?}",arr);
// }

// fn write_arr(mut arr1:[&str;3]){
//     arr1[0]="fellow";
//     println!("arr1={:?}",arr1);

// }


fn main()
{


    
}