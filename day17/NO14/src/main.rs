fn main() {
    let input = "100";

    let number: i32 = input.parse().unwrap();
    //parse()...文字を数字にする
    //unwrap()...絶対に数字が入ってるからいいよー
    println!("numberの2倍は{}です",number*2);
}
