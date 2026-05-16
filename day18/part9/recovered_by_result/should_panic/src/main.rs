//起こるべくして起こるエラーはresultを使う
//よくない時はpanic!を使う


fn main(){
    pub struct Guess{
        value: i32,
    }

    impl Guess{
        pub fn new(value: i32) -> Guess{
            //変なのが来たらここで追い出す
            if value < 1 || value > 100{
                panic!("1から100の範囲じゃないとダメだよ");
            }

            Guess{value}
        }

        pub fn valu(&self) -> i32{
            self.value
        }
    }
}