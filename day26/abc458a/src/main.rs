use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        s: Chars,
        n: usize,
    }

    let l = s.len();

    for i in 0..l{
        if n<=i && i<l-n{
            print!("{}", s[i]);
        }
    }
    println!();
    println!("hello");
}


