fn main() {
    let today_shift: Option<&str> = None;;

    match today_shift{
        Some(time) => println!("今日のバイトは{}だよ！",time),
        None => println!("入力されてないよ"),
    }
}
