fn main() {
    println!("Hello, world!");

    //string can change so it is dynamically assign so heap is assigned 
    let s  = String ::from("hello"); //string literals
    println!("s = {s}");
    
    let mut s1  = String ::from("hello"); //string literals
    s1.push_str("world" ); //since it is in heap so it can grow and shrink
    //if it was in stack , it cant grow and shrink , it need to be of same fixed size
    println!("s = {s1}");

    let mut x = 5;
    let y = x ; //x value is copied 
    println!("x is {x} and y is {y}");
    x =10;
    println!("x is {x} and y is {y}");
}
