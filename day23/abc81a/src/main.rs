use proconio::input;
use proconio::marker::Chars;


fn main(){
    input!{
        s: Chars,
    }
    //proconio::marker::Charsで文字を一文字ずつの配列にしてくれる
    let mut cnt = 0;
    for i in s{
        if i == '1'{
            cnt += 1;
        }
    }
    println!("{}", cnt);
}