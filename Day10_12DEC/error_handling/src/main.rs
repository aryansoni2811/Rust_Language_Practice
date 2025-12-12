use std::fs::File;

fn main() {
    println!("Hello, world!");

    //panic!("crashed"); // it panic the code and exit the code  //it tells the coed from which line the error occured 

    //recoverable error
    let r = match divide(4 , 0){
        Ok(num) => num ,
        Err(err) => {
            println!("{err}");
            -1
        }
    };
    println!("{r}");



    let greeting_file_result = File :: open("hello.txt");
    let greeting_file :File =  match greeting_file_result {
        Ok (file ) => file , 
        Err (error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file) => file ,
                Err(e) => panic!("problem creating the file : {e:?}"),  
            },
            _=> panic!("Something went wrong {error:?}"),
        },
    };
}

fn divide(x: i32 , y: i32) -> Result<i32 , String> {   //here result will return i32 value and if fail it will return string
    if y == 0{
        return Err (String:: from("Please dont divide by zero")); 
    }
    Ok(x/y)
        
}
