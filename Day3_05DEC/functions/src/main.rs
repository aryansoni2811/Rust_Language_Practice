fn main() {
    println!("Hello, world!");
    another_function();
    my_function(6);

    let y :i32 = {
        let x : i32 =3;
        x+1     //it will return x+! //semicolon should not be there 
    };
    println!("y is {y}");

    let y = my_function1(1,2);
    println!("y is {y}");
}

fn another_function(){
    println!("another function");
}

fn my_function(x:i32){
    println!("x value {x}" );
}

fn my_function1(x:i32 , j :i32) -> i32 {
    //return 10;
    x+j
}
