pub fn get_task_status(days_left: i32) -> String{
    if days_left < 0{
        String::from("期限切れです")
    }else{
        format!("残り{}日です",days_left)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal_logic_works(){
        assert_eq!(get_task_status(-1), "期限切れです");
    }
}