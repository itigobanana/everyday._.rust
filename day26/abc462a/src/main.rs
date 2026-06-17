use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        s: Chars,
    }

    for c in s{
        if c.is_ascii_digit(){
            print!("{}", c);
        }
    }
    println!();//改行
}