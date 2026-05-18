/*
#[test] をつけることで、テスト用の関数であることを宣言
⭐️計算結果があっているかを確認するための強力なマクロ
1️⃣assert!(条件)
かっこの中がtrueなら合格、falseならパニック
2️⃣assert_eq!(左、右),assert!(a == b)
左と右が等しいなら合格、それ以外は左と右の具体的な中身を教えてくれる
3️⃣assert_ne!(左、右)
左と右が等しくないなら合格。絶対この値にはなってほしくないという確認に使う


fn main(){
    失敗した時のエラーメッセージを自分で付け足すことができるよ。
    assert!(
        result.contains("Carol"),
        "挨拶の中に名前がないよ！実際の中身はこれだった：'{}'",result
    )
}

⭐️正しくエラーになるか？をテストしたい時は「#should_panic」をつける。
→ただパニックすれば合格としてしまうと、意図しない別のエラーで落ちた時も合格になってしまう。
→ #[should_panic(expected = "100以下でなければなりません")]
  と書いておけば、その文字を含んだエラーメッセージで落ちた時だけ合格！っていう厳密なテストができる

⭐️ Result<T, E> →パニックさせないテスト
#[test]
fn it_works() -> Result<(), String>{
    if 2 + 2 == 4{
        Ok(())
    }else{
        Err(String::from("2+2が4じゃないなんておかしいよ"))}
}

*/



pub fn is_deadline_missed(days_left: i32) -> bool{
    days_left > 0
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_deadline(){
        assert_eq!(is_deadline_missed(-1),true);
        assert_eq!(is_deadline_missed(3),false);
    }
}
