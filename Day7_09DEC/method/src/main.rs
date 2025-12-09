#[derive(Debug)]

struct Rectangle{
    width : u32 ,
    height : u32,
}

impl Rectangle {
    fn calulate_area(&self)-> u32 {   //self is like this keyword
        self.height * self.width
    }

    fn can_hold(&self , other : &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }

    fn square (side : u32) -> Self {   //here self is rectangle only 
        Rectangle{
            width : side,
            height : side,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let rect = Rectangle {
        width : 32 , 
        height : 50
    };

    let rect2 = Rectangle{
        width : 9,
        height : 9,
    };

    rect2.calulate_area();

    println!("area is {}", rect.calulate_area());


    println!("can hold : {}" , rect.can_hold(&rect2));

    let sq = Rectangle :: square(5);
    println!("{:?}" , sq);


}
