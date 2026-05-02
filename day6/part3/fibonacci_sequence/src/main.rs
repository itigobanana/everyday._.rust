fn main() {
    let ans = seek_fibonacci_sequence(6);
    println!("{ans}");
}

fn seek_fibonacci_sequence (n: i32) -> i32 {
    let mut now = 1;
    let mut pre = 1;
    for i in 1..n+1{
        if i==1 || i==2{
            continue;
        }
        else{
            let tmp = pre + now;
            pre = now;
            now = tmp;
        }
    }
    now
}