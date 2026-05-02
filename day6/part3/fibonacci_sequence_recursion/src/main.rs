fn main() {
    let ans = f(8);
    println!("{ans}");
}

fn f(n: usize) -> usize{
    if n==1 || n==2{
        return 1
    }
    f(n-1) + f(n-2)
}

//すっきり！めっちゃかっこいいコード！
//数列のとき再帰の可能性をゴールから逆算して考えたらいいかも！
