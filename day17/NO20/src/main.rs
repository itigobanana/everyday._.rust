fn main() {
    let numbers = vec![1,2,3,4,5];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("元の数字:{:?}",numbers);
    println!("2倍になった数字:{:?}",doubled);
}
