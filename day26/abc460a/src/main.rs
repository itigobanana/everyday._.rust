use proconio::input;

fn main(){
    input!{
        n: i32,
        mut m: i32,
    }

    let mut cnt = 0;

    loop{
        m = n%m;
        cnt += 1;

        if m==0{
            break
        }
    }

    println!("{}", cnt);
}