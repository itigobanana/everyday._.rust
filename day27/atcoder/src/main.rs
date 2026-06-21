use proconio::input;
// use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        mut takahasi: [(i64, i64); n],
        q: usize,
        mut t: [i64; q], 
    }

    takahasi.sort_by(|a,b| b.1.cmp(&a.1));

    let mut q_idx = Vec::new();
    for i in 0..q{
        q_idx.push((t[i],i));
    }
    q_idx.sort_by(|a,b| b.0.cmp(&a.0));

    let mut ans = vec![0; q];
    let mut maxh = 0;
    let mut t_idx = 0;

    for (time, idx) in q_idx{
        while t_idx < n && takahasi[t_idx].1 >= time+1{
            if takahasi[t_idx].0 > maxh{
                maxh = takahasi[t_idx].0;
            }
            t_idx += 1;
        }
        ans[idx] = maxh;
    }

    for a in ans{
        println!("{}", a);
    }
}