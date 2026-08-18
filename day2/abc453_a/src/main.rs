use proconio::input;

fn main() {
    input! {
        n:usize,
        s:String,
    }

    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < n && chars[i] == 'o' {
        i += 1;
    }

    if i == n {
        println!("");
    } else {
        let result: String = chars[i..].iter().collect();
        println!("{}", result);
    }
}



