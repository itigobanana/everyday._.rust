use test_systematization::get_task_status;

#[test]
fn app_works_froom_outside(){
    assert_eq!(get_task_status(5),"残り5日です")
}