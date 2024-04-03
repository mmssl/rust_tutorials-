#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {

    let xx: [i32; 5] = [1,2,3,4,5];
    println!("first array is {:?}", xx);
    let hello = "Hello Rust!";
    println!("{}", hello);
    let name = String::from("Peter");
    let age = 24;
    let peter = Person{name, age};
    println!("{:?}", peter);
}
