fn main() {
    // println!("Hello, world!");

    let mut hello = String ::from ("नमस्ते");

    hello.push_str(" in hindi");
    hello.push('.');
    println!("{hello}");

    let s1 = String:: from("hello");
    let s2 = String:: from("world");
    let s3 = s1 + &s2 ;

    println!("{s3}");


    let s4 = String:: from("hello");
   // let h = s1[0] ;


    for c in hello.as_bytes(){
        println!("c = {c}");
    }
    for c in hello.chars(){
        println!("c = {c}");
    }
}
