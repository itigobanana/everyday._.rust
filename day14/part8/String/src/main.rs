fn main() {
    //rustの文字列は、ただの文字の集まりではなく、UTF-8というルールで圧縮されたバイトの塊である

    /*
    1️⃣文字列スライス(&str)とStringの違い
    ・文字列スライス(&str) = 壁のポスター
    let s = "Hello"のように書いた時、これはプログラムの中に直接埋め込まれた変更不可能なポスターみたいなもの。
    サイズを広げたり、文字を付け足したりできない。
    ・String = 自由帳
    let s = String::from("Hello")のように書いた時、ヒープ領域に作られた自由帳。
    後から文字を付け足したり、サイズを広げたりできない。
    */

    //文字を追加結合する
    //後ろにくっつける(push_str,push)
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    //綺麗に繋げる(format!)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s = format!("{}---{}",s1,s2);

    println!("{}",s);

    /*
    pythonやC＋＋では文字列から一文字めを取り出すときs[0]と書くが、rustでは許されない。
    これはUTF-8に原因がある。
    英語の'A'は1byte,日本語の'あ'は3byteあり、s[0]の1byte目のみを返してしまう。危険！
    ここで、人間が読める文字として分解する命令(.chars())メソッドを使う。
     */

    let word = "こんにちは";

    for c in word.chars(){
        println!("{}",c);
    }
}
