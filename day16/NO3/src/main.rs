fn main() {
    let message = String::from("いつもありがとう！");

    send_message(&message);

    println!("再送：{}",message);
}

fn send_message(msg: &String){
    println!("再送完了：{}",msg);
}