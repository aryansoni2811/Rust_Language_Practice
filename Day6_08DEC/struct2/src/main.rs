#[derive(Debug)]

struct Rectangle {
    width : u32,
    height : u32,
}


fn main() {
    println!("Hello, world!");
    // let w =100;
    // let h = 200;
    let rect = Rectangle {
        width : 32 , 
        height : 32 ,
    };
    let area = dbg!(calulaye_area(&rect));
    println!("area is {area}");
    //println!("area is {rect}"); /// this will give error so will use debug errorr 
    println!("area is {:#?}" , rect);  //{:?} is normla and {:#?} is pretty debug 
}

// fn calulaye_area(width : u32 , height:u32){  

//     width *height
// }
//now here user dont knwo whether firt paramater is width or height so..


// fn calulaye_area(dimensions :(u32 ,u32)) -> u32{  
//     let (width : u32  , height :32) = dimensions;
//     width *height
// }

fn calulaye_area(rect : &Rectangle) -> u32{  
    rect.width * rect.height
}