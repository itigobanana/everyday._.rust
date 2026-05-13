fn main() {
    let tasks = ["読書","編み物","勉強"];

    println!("今日の予定はこちら！");

    for i in 0..3{
        println!("{:?}",tasks[i]);
    }
}
