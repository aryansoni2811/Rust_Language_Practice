fn main() {
    let a : i8 =-2;  //signed int so -ve sign is allowed
    //let a :u8 = -2  //errro
    let a :u8 = 254u8 ;
    let c :u32 = 1_00_00_000; // it is still a valid number as 10000000
    let a :u8 = b'A'; //goive byte value 65 asci table of A

    let a = 3.0; //default type is f64

    let a = 4/2 ; //value 2
    let a = 5/2 ;// as type is not specified and it will i 32 so 2.5 will be changed to 2 answer 
    let a : f32 = 5_f32 / 2_f32; //2.5
    let a : f32 = 5.0 / 2.0 ; //2.5

    let c = 'c'; //char are with single quotes
    let c = "c"; // it will infer as string

    let tup = (500 , 6.4 , 1 , 'a'  , true , "fbadhfba");
    println!("{}"  , tup.0);
    println!("{}"  , tup.1);
    println!("{}"  , tup.2);
    println!("{}"  , tup.3);
    println!("{}"  , tup.4);

    let arr :[i32; 5] = [1,2,4,4];
    let arr = [10;5] ; // it will have array of size 5 wih all value as 10
}
