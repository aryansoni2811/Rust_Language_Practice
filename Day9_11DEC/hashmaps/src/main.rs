use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    let mut scores:  HashMap<String , i32> = HashMap::new();

    scores.insert(String ::from("blue")  , 10);
    scores.insert(String ::from("yellow")  , 20);

    println!("{:?}"  , scores);


    let sc = scores.get(&String :: from("blue")).unwrap_or(&0);
    println!("{sc}");

    scores.entry(String ::  from ("red")).or_insert(50) ; //if red doesnt exist in score its value will be inserted 


    

    let text = "adafagaf fdhddhjdhdh adafagaf" ;
    let mut map = HashMap :: new();

    for word  in text.split_whitespace(){
        println!("{word}");

        let count = map.entry(word).or_insert(0);
        *count+=1;
    }


    for (key , value ) in &map {
        println!("{:?} => {:?}" , key , value);
    }

}
