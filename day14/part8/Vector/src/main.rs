fn main(){
    //空白から作る場合(Vec::new())
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(7);

    //最初から中身を入れて作る場合(vec![]マクロ)
    let v = vec![1,2,3];
    //絶対にここにあるはずだ！ってと時の添字
    let third = &v[2];
    //もしここに存在しなかったらクラッシュ

    let third = v.get(2);
    //あるかどうかわからない時
    //これを使うと結果はOptionで返ってくる
    //ある場合：Some(&値)
    //ない場合:None



    //最初の要素を参照したまま最後に要素を追加しようとするとエラーになる

    /*
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];

    v.push(6);
    println!("{}",first);
    */

    //->ベクタはパソコン内のヒープに隙間なく一列に並べて保存されているので、
    //もしpushで要素を追加しようとした時、今の場所がパンパンでもう後ろにスペースがない！
    //ってなると、もっと広い別の場所に全員まとめて引っ越す。模試参照しっぱなしだったらバグる
    //その前にrustは止めてくれる！

    //ベクタは同じ方しか入れられないが、文字と数字を混ぜて入れたいときはEnum!

    enum SpreadsheetCell{
        Int(i32),
        Float(f46),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ]


    //vec![]で作って、push()で追加して、get()でとる！
}