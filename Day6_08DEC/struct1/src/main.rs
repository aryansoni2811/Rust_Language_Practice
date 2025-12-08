struct User{
    active : bool , 
    username : String ,
    email : String,
    sign_in_count : u64,
}

struct Color (u8, u8, u8);
struct Point (u8, u8, u8);


fn main() {
    println!("Hello, world!");

    let user1: User = User {
        email : String::from("absdf@gmail.com"),
        username: String::from("dsfsgsdfg"),
        active: true,
        sign_in_count : 0,
    };

    let mut user2: User = User {
        email : String::from("absdasfgsgf@gmail.com"),
        username: String::from("dsfsdsfsdfgsdfg"),
        active: true,
        sign_in_count : 0,
    };
    let mut user3: User = User {
        email : String::from("absdasffdfdsfsfsgsgf@gmail.com"),
        ..user1   //it will have same value  of other feild as user1
    };


    println!("{}",user1.active);
    println!("{}" ,user1.email);



    user2.email = String :: from ("dffsaf");


    let red =  Color(100 , 0 , 0);
    set_bg(red);
    let point1 = Point(1 , 2 , 3);
    set_point(point1); // here we can also pass red in bracket as it u8 so it is ptroblem so we make tupple struct above then 

}


fn set_bg (color : Color){
    println!("{} {} {}" , color.0 , color.1 , color.2);
}
fn set_point (point :Point){ // now we alsways have to pass point in it not color 
    println!("{} {} {}" , color.0 , color.1 , color.2);
}

