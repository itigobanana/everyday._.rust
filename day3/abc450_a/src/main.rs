use proconio::input;

fn main() {
    input!{
        n:usize,
    }

    let mut v = Vec::new();

    for i in (1..n+1).rev() {
        v.push(i);
    }

    println!("{}", v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(","));
}
