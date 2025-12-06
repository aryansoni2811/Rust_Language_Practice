fn main() {
    println!("Hello, world!");
    let number =3;
    if number < 5{
        println!("true");
    }else{
        println!("false");
    }

    let y : bool = is_even(5);
    if y {
        println!("fn returned true");
    }
    println!("{y}"  );


    //infinite loop
    //loop{
        //println!("asda");
    //}

    let mut a = 3 ;

    loop {
        println!("{a}");
        if  a == 10 {
            break;
            //break 70 ; // it will actucally return the value 70 to the varaible assign for the loop    
        }
        a = a+1;
    }



    let mut x  = 3; 

    while x  != 0 {
        println!("{x}");
        x =x -1;
    }




    let a : [i32 ; 4 ] = [1 ,2  ,3 ,4 ];

    for element in a {
        println!("{element}");
    }


    for x in 1..10 {
        println!("{x}");
    }
    for x in 1..=10 {    //10 is incusive here
        println!("{x}");
    }
    for x in (1..10).rev() { //reverse order
        println!("{x}");
    }

}

fn is_even(x : i32) -> bool{
    if x%2 == 0{
        true 
    }else{
        false
    }

}
