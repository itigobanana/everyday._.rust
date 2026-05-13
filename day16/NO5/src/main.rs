fn main() {
    let judge = "WA";

    match judge{
        "AC" => println!("正解！"),
        "WA" => println!("不正解！"),
        "TLE" => println!("実行時間オーバー！"),
        _ => println!("何これ！"),
    }
}
