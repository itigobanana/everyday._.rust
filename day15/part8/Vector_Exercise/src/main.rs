//整数のリストが与えられ、ベクタを使ってmedian(ソートされた時に真ん中に来る値)、
//mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v = [1,1,2,1,3,4,3,2,3,4].to_vec();

    v.sort();
    println!("sorted_v:{:?}",v);

    let amount = v.len();
    println!("v_len:{}",amount);

    let median: f64;
    if amount%2 == 0{
        median = (v[amount/2] as f64 + v[amount/2-1] as f64)/2.0;
    }else{
        median = v[amount/2] as f64;
    }

    println!("median:{}",median);

    let mut numbers = HashMap::new();

    for &num in &v{
        let count = numbers.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;

    for (key, value) in &numbers{
        if *value > max_count{
            max_count = *value;
            mode = *key;
        }
    }


    println!("mode:{}",mode);
}
