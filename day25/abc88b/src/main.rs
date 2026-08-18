use proconio::input;

fn main(){
    input!{
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();

    let mut alice = 0;
    let mut bob = 0;

    for i in 0..n{
        if i%2==0{
            alice += a[i];
        }else{
            bob += a[i];
        }
    }
    
    println!("{}", alice-bob);
}