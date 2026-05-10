1️⃣パスには絶対と相対がある。絶対パスを使うと、道順が狂いにくい。
2️⃣一つ前のフォルダに戻る「..」はrustのモジュールでもsuperを使うとできる
例：super::deliver_order();
3️⃣公開するにはpubをつけるが...

❌
pub mod hosting{
    fn add_to_waitlist(){}
}

と書くとエラー。
mod hostingにpubをつけたからといって、中にある関数まで自動的に公開されるわけではない！

⭕️
pub mod hosting{
    pub fn add_to_waitlist(){}
}

どっちにもpubをつけよう！

4️⃣構造体(struct)とEnumで公開のされ方は異なる

構造体(struct)は一つずつ細かく設定→

pub struct Breakfast{
    pub toast: String, //公開
    seasonal_fruit: String,//非公開
}

Enumは外箱を開けたら全部公開→

pub enum Appetizer{
    Soup,
    Salad,
}