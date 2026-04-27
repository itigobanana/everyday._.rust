use proconio::input;

fn main() {
    input!{
        m: usize,
        d: usize,
    }

    if m == d {
        if m % 2 == 1 && 1 < m && m < 10 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if m == 1 && d == 7 {
        println!("Yes");
    } else {
        println!("No");
    }
}