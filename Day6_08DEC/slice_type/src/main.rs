fn main() {
    println!("Helloworld!");

    let s  = String :: from("HelloWorld");

    let res = find_first_word(&s);
    println!("res os {res}");


    let s  = String :: from("Hello World");
    let hello = &s[0..5]; //[..5] is also valid
    let world = &s[6..11]; //[6 .. ] is also valid till last

    println!("{hello} {world}");
}


// fn find_first_word(input : &String) -> usize {
//     let s :&[u8] =input.as_bytes();

//     for (i , &item) in s.iter().enumerate(){
//         if item == b' ' {
//             return i 
//         }
//     }  

//     s.len()
// }


fn find_first_word(input : &String) -> &str {
    let s :&[u8] =input.as_bytes();

    for (i , &item) in s.iter().enumerate(){
        if item == b' ' {
            return &input[..i] 
        }
    }  

    &input[..]
}
