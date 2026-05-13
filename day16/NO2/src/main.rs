fn main() {
    let my_score = 75;
    println!("プログラミングの単位は...{}!",check_grade(my_score));
}

fn check_grade(score: i32) -> String{
    if score >= 60{
        "合格".to_string()
    }else{
        "再履修".to_string()
    }
}
