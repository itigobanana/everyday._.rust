#[derive(Debug)]
struct Task{
    name: String,
    is_done: bool,
}

fn main(){
    let mut my_tasks = vec![
        Task{name: String::from("微積の課題"),is_done: false},
        Task{name: String::from("バイト"),is_done:false},
        Task{name: String::from("エラー修正"),is_done:false},
    ];

    complete_task(&mut my_tasks, "微積の課題");

    println!("更新後のタスク：{:#?}",my_tasks);  
}

fn complete_task(tasks: &mut Vec<Task>, target_name: &str){
    for task in tasks{
        if task.name == target_name{
            task.is_done = true;
        }
    }
}