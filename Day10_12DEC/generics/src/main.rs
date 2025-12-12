struct Point<T> {
    x : T  ,
    y : T ,
}

fn main() {
    println!("Hello, world!");

    let list =  vec![1, 2,4 ,5 ,6];  // vec is also generic type 
    let list1 =  vec![1.0, 2.0,4.0 ,5.0 ,6.0];  // vec is also generic type 
    let l = largest (&list);
    println!("{l}"); 
    let l = largest (&list1);
    println!("{l}"); 


    let point_a = Point{ x : 10 , y : 10};
    let point_b = Point{ x : 10.0 , y : 10.0}; // here bothe type of x and y should be same it should not be different 



}

fn largest<A>(list : [&A]) -> &A {
    let mut result : &A  =&list[0] ;

    for item: &A in list {   // whatever type we are pasiing to largest it is infer to A variable
        if item > result {
            result = item ;
        }
    }

    &result
}
