// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin{
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin{
//         Coin::Penny => {
//             println!("Lucky Penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32>{
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }

// fn main(){
//     let five = Some(5);//5の入った箱
//     let six = plus_one(five);//5の入った箱をplus_one関数に渡してる
//     let none = plus_one(None);//NoneはNone!
// }

fn main(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
}