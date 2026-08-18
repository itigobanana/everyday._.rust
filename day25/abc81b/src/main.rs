use proconio::input;

fn main(){
    input!{
        n: usize,
        mut a: [i32; n],
    }
    let mut cnt = 0;

    loop{
        let mut even = true;

        for i in &a{
            if i%2 != 0{
                even = false;
            }
        }

        if even == false{
            break
        }

        for j in &mut a{
            *j /= 2;
        }

        cnt += 1;
    }

    println!("{}", cnt);
}