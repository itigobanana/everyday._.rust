fn main() {
    let text = String::from("I love rabbit");
    let love_part = &text[7..];
    println!("切り取った文字：{}",love_part);
}
