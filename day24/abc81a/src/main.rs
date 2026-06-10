use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        s: Chars
    }
    let mut cnt = 0;
    for i in 0..3{
        if s[i] == '1'{
            cnt += 1;
        }
    }
    println!("{}", cnt);
}