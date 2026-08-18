use proconio::input;

fn main(){
    input!{
        n: usize,
        a: usize,
        b: usize,
    }

    let mut cnt = 0;

    for i in 1..=n{
        let mut now = i;
        let mut digit_sum = 0;
        
        while now > 0{
            digit_sum += now%10;
            now /= 10;
        }

        if a <= digit_sum && digit_sum <= b{
            cnt += i;
        }
    }

    println!("{}", cnt);
}