use std::io;

fn main() {


   /* let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1; // shadowing
    println!("The value of x is: {x}");
    let x = x * 2;
    println!("The value of x is: {x}");

    let spaces = "   "; // this is string type 
    let spaces = spaces.len(); // this is number type 
*/

    let a = [1,2,3,4,5];
    println!("enter array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read");
    let index: usize = index
        .trim()
        .parse()
        .expect("Failed to read");

    let element = a[index];
    println!("The value of element at index {index} is {element}");
    


}
