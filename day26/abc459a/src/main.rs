use proconio::input;

fn main(){
    input!{
        x: usize,
    }

    let s: Vec<char> = "HelloWorld".chars().collect();

    for c in 0..10{
        if c != x-1{
            print!("{}", s[c]);
        }
    }

    println!();
}