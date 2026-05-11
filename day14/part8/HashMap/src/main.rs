use std::collections::HashMap;

fn main() {
    /*
    ハッシュマップとは、名前シール付きのロッカー
    ハッシュマップを使うには...
    宣言が必要(use std::collections::HashMap;)とかく
    HashMap::new()でからのマップを作り、.insert(キー、値)でデータを入れる
    キー同士、あたい同士は全て同じ型でないといけない
     */

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //scores.get(&team_name)...キーの参照を渡して検索。Option型で返ってくる
    //.copied()...getが返してくるのは値の参照なので、これをコピーして普通の実体(i32)に変換
    //.inwrap_or(0)...もし箱の中身が空っぽだったら代わりに0を使ってねっていう命令

    //for (key,value) in &scoresのようにループで中身を出すこともできるけど、取り出される順番がバラバラになる

    //ハッシュマップにデータを入れるとき、所有権のルール発動
    //Stringのような所有権を持つデータは、insertした瞬間にハッシュマップにムーブしてしまう

    let name = String::from("Blue");
    scores.insert(name, 10);
    //ここからnameは使えない

    //すでにデータが入っているマップを更新する時どう処理するか？？
    //1️⃣上書き。普通に同じキーで.insert()を2回呼ぶと、古いデータは消滅し、新しいデータに完全に上書きされる
    //2️⃣まだない時だけ追加する。データがまだ存在しない場合だけ初期値を入れたい時。

    //Yellowがなければ50を入れる（ある時は何もしない）
    scores.entry(String::from("Yellow")).or_insert(50);

    //3️⃣古い値を利用して更新する。文章内にある単語の数を数えるときに使うよ

    //単語が初めて出た時は0を入れ、その場所をcountに渡す
    let count = map.entry(word).or_insert(0);
    //参照外しをして、その場所の数値を直接+1する
    *count += 1;
}
