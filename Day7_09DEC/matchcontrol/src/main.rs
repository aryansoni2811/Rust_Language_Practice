#[derive(Debug)]

enum Usstates{
    Alabama , 
    Alaska, 
}

enum Coin {
    Penny , 
    Nickel , 
    Dime , 
    Quater , 
}


fn main() {
    println!("Hello, world!");

    let coin =  Coin :: Penny ;
    println!("Value is {}"  , value_in_cents(coin));
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin :: Penny => 1 ,
        Coin :: Nickel => 5 ,
        Coin :: Dime => 10 ,
        Coin :: Quater(Usstates::Alabama) => {
            println!("HELLO FROM ALSASKA ");
            34
        },
        Coin :: Quater(state : Usstates) =>{
            println!("got q of value {:?}" , state);
            25
        } ,
    }
}
 