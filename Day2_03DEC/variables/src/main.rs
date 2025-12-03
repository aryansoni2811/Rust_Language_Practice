fn main() {
    println!("Hello, world!");

    const PI : u8= 3;
    println!("pi value {PI}");


    let apples = 10;
    let apples = 20; //shadowing remember use let otherwise there will be problem of mutablity
    let apples = true; //shadowing also allowed to change the type of variable 
 
    println!("apples {apples}");


    let x = 5; 
    let x = x+1;

    {
        let x = x *2 ;
        println!("x : {x}");
    }

    println!("x : {x}");
}
