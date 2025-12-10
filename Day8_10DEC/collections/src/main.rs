fn main() {
    println!("Hello, world!");

    //let mut vec  = Vec :: new ();
    let mut vec : Vec<i32> = vec![1 , 2, 3 , 4 ,5];
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4); // in vector all the elemnt need to be of same type 

    println!("{:?}" , vec);
    //let fourth_index = vec[3];  // if we try to access out of bound like 30 th index program will crash
    // so 

    let fourth_index = vec.get(3).unwrap(); // this wil return actually optional value 
    println!("{fourth_index}");

    let fourth_index = match vec.get(30){
        Some (value : &i32) => value ,
        None => {
            println!("the index is out of bound");
            &-1
        }
    };

    println!("{fourth_index}");

}
