use proconio::input;

fn main(){
    input!{
        a: i32,
        d: i32,
    }

    if d >= a{
        println!("Yes");
    }else{
        println!("No");
    }
}