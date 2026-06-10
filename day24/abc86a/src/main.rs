use proconio::input;

fn main(){
    input!{
        a: i32,
        b: i32
    }
    let ab = a*b;

    if ab%2 == 0{
        println!("Even");
    }else{
        println!("Odd");
    }
}