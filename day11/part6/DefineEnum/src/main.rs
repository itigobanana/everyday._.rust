// fn main(){
//     enum IpAddrKind{
//         v4,
//         v6,
//     }
    
//     struct IpAddr{
//         kind: IpAddrKind,
//         address: String,
//     }
    
//     let home = IpAddr{
//         kind: IpAddrKind::v4,
//         address: String::from("127.0.0.1")
//     };
    
//     let loopback = IpAddr{
//         kind: IpAddrKind::v6,
//         address: String::from("::1"),
//     };
// }


// fn main(){
//     enum IpAddr{
//         V4(u8,u8,u8,u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127,0,0,1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }



// fn main(){
//     enum Message{
//         Quit,
//         Move{x: i32, y: i32},
//         Write(String),
//         ChangeColor(i32,i32,i32),
//     }

//     //
//     struct QuitMessage;
//     struct MoveMessage{
//         x: i32,
//         y: i32,
//     }

//     struct WriteMessage(String);
//     struct ChangeColorMessage(i32,i32,i32);
//     //


//     impl Message{
//         fn call(&self){
//             println!("successed!");
//         }
//     }
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


//NULLはバグの最大の原因
//
fn main(){

    // enum Option<T>{
    //     None,
    //     Some(T),
    // }

    //optionという箱📦
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //むき出し(x)+箱に入ったやつ(y)は計算できない。
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}