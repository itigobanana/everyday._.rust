use std::collections::HashMap;

fn main(){
    let results = vec!["AC","WA","WA","TLE","AC","WA"];

    let mut counts: HashMap<&str, i32> = HashMap::new();

    for result in results{
        *counts.entry(result).or_insert(0) += 1;
    }

    for (res, count) in counts{
        println!("{}:{}回",res,count);
    }
}