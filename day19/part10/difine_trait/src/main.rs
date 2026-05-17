fn main() {
    //何でもかんでもジェネリックに入れたら、計算できない！などのエラーになるかも。。。
    //それを防ぐために条件をつけるのがトレイト！




    //Summaryという資格を作る
    pub trait Summary {
        //この資格を持つものは、絶対にsummarizeメソッドを持たなければならない
        fn summarize(&self) -> String;
    }

    //Tweet型にSummary資格を与える
    impl Summary for Tweet {
        fn summarize(&self) -> String{
            format!("{}: {}",self.username,self_content)
        }
    }



    //資格を作る時点で、デフォルトの動きを決めておくこともできる
    pub trait Summary{
        fn summarize(&self) -> String{
            String::from("(もっと読む。。。)")
        }
    }
    //デフォルトでいい場合は{}で終わる
    impl Summary for NewsArticle{}



    //Summary資格を持っている型なら、なんでも受け入れるよ！
    pub fn notify(item: &impl Summary){
        println!("速報！{}",item.summarize());
    }

    //上のコードと比較！<T:Summary>は省略可能
    pub fn notify<T:Summary>(item: &T){
        println!("速報！{}",item.summarize());
    }

    //引数が二つある、尚且つ二つの方は同じでないといけない時は、省略前の書き方がいい
    pub fn notify<T: Summary>(item1: &T, item2: &T){
        println!("速報！{}",item.summarize());
    }

    //ようやくできて、かつ出力できる型、のように複数の資格を要求するときは+で繋ぐ
    pub fn notify(item: (&impl Summary + Display))

    //関数の戻り値に-> impl Summaryと書くことで、Summary資格を持っている何かを返すと宣言できる
    //返す値は絶対に一種類じゃないとダメ


    //Tが特定の資格を持っている時だけ、メソッドを追加する
    impl<T: Display + PartialOrd> Pair<T>{
        fn cmp_display(&self){}
    }
}
