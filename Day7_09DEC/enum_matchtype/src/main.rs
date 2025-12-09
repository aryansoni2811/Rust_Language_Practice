#[derive(Debug)]
// enum IPAddrKind{
//     V4  ,
//     V6,
// }

// struct IPAddress {
//     address : String, 
//     kind : IPAddrKind,
// }


// fn main() {
//     println!("Hello, world!");
//     // route("1.2.3.4" , IPAddrKind ::V4);


//     let google = IPAddress{
//         address :  String :: from ("1,2,3,4"),
//         kind : IPAddrKind:: V4,
//     };
//     route(google);
// }


// // fn route (ip : &str , kind  : IPAddrKind){
// //     println!("{ip}  of kind {kind:?}");
// // }
// fn route (ip : IPAddress){
//     println!("{}  of kind {:?}" , ip.address , ip.kind);
// }

//--------------------------------------------------------------------------------

// enum Ipaddr {
//     V4 (u8 , u8 , u8 , u8) ,
//     V6 (String), 
// }


// fn main (){
//     let home = Ipaddr :: V4(1,2,3 ,4);
//     route(home);
//     let home1 = Ipaddr :: V6(String::from("1.2.3.4"));
//     route(home1);
// }


// fn route (ip : Ipaddr){
//     println!("{:?}" , ip);
// }


//--------------------------------------------------------------------------------

enum Message {
    Quit  , 
    Move {x : i32 , y : i32} ,
    Write(String) ,
    ChangeColor(i32 , i32 , i32),
}    

