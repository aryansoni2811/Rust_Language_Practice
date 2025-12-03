use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("Guess the Number");
    let secret_number : i32 = rand::thread_rng().gen_range(1..=100);


loop {
  
    println!("Please input your guess.");

    let mut guess : String = String :: new();  // here the guess is immutable string so we cant add any string
    //then add mut to make it mutable
    //guess.push_str("aryan");  //error 


    
    //generate random number form 1 to 100
    //println!("secret number is {secret_number}");
    //== is used for inclusive range i.e 1 and 100 is included

    
    
   io::stdin().read_line(&mut guess).expect("failed to read input");  //this is for taking input
    //but still now guess is not given the refernece of mutable here so still it will give error 
    //after adding &mut it will reference as mutable so it can chnage value

    let guess  : i32 = guess.trim().parse().expect("failed to parse"); 
    
    //match is kind of switch statement 
    //and in match we always have to cover all the possible situation and condition not like switch case 
    match guess.cmp(&secret_number){
        Ordering::Equal => println!("you won !"),
        Ordering::Greater => println!("too big"),
        Ordering::Less => println!("too small"),
    }

    println!("you guess the {}"  , guess);
}   
}
