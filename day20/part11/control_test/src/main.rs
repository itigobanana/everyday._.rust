/*
⭐️「--」の役割
オプションには2種類ある。
・cargo test [option] = Cargo自体への命令
・cargo test -- [option] = 作られたプログラムへの命令

⭐️デフォルトでは、Rustはテストを並行して走らせるので高速で実行できるが、複数のテストが同時に同じ
ファイルに書き込みをして、データがぐちゃぐちゃになって失敗する事故が起きることがある。
→ cargo test -- --test-threads=1 を使うことで、一個ずつ順番に実行させる

⭐️println!で途中経過を出力するコードを書いても、
テストが合格（Ok)した時は、ターミナルが散らからないように、Rustが勝手に出力を隠してしまう。
→ cargo test -- --show-output 　を使うことで、合格した時もprintln!の中身を出力する

⭐️たくさんのテストを一つのバグを治すために毎回全部実行していたら時間が勿体無い。
引数に名前の一部を渡すと、その文字を含むテストだけをフィルタリングして実行できる。
→ cargo test add を使うことで、addを含むテスト「add_oneなど」だけが実行される。

⭐️重いテストには、関数の上に#[test]とセットで#[ignore]をつける。
こうすると、普段のcargo testでは自動的に無視されるようになるよ
→ cargo test -- --ignored で、無視している重いテストだけを実行
→ cargo test -- --include-ignored で、全部のテストを実行
*/


pub fn add(a: i32, b:i32) -> i32{
    a + b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn math_add_works(){
        println!("[出力] math_add_works を実行中...");
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn math_add_fails(){
        println!("[出力] math_add_fails を実行中...");
        assert_eq!(add(2,3), 4);
    }

    #[test]
    fn string_check(){
        println!("[出力] string_check を実行中...");
        assert!(true);
    }

    #[test]
    #[ignore]
    fn heavy_caluculation(){
        println!("[出力] 時間のかかる思いテストを実行中...")
    }
}