use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("微積"),90);
    scores.insert(String::from("プログラミング"),40);

    println!("成績データ:{:?}",scores);
}