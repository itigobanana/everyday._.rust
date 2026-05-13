fn main() {
    let homework = String::from("プログラミング");

    print_task(&homework);

    println!("{}の課題終わり！",homework);
}

fn print_task(task: &String){
    println!("今日の課題は{}",task);
}
