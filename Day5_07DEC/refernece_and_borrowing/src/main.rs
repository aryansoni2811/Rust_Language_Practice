fn main() {
    println!("Hello, world!");
    let s : String = String :: from ("asfsvv00");
    let len : usize = calculate_len(&s); //& allows you to refer to some value without taking ownership
    println!("{s} {len}" );

    let mut s1 : String = String :: from ("asfsvv00");
    s1.push_str("fsgfsfb");   // here is it mutbale 
    let len1 : usize = calculate_len1(&mut s1);
    println!("{s1} {len1}" );

}

fn calculate_len(s: &String) -> usize{   //here &string is borrowing we are taking referneces , if it is only string it is move 
    let result : usize = s.len();
    //s.push_str(".vnsdfdf"); //it will give erro // becaue it dont have ownership so it cant change the variable it only boorws the thinsg
    result

}


fn calculate_len1(s: &mut String) -> usize{   //here &string is borrowing we are taking referneces , if it is only string it is move 
    let result : usize = s.len();
    s.push_str("DSSFGFDG"); // NOW SINCE IT IT MUTABLE REFERNCE SO IT CAN CHANGE 
    result

}
