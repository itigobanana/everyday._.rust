/*
1️⃣
・単体テスト　一つの関数が、想定通りに動くか細かくチェックする（内側）
・結合テスト　複数の部品を組み合わせて、ユーザーと同じ目線で全体が正しく動くかチェックする（外側）

2️⃣単体テストのルール　→　テスト対象のコードを全く同じファイルにかく
・　#[cfg(test)]　→ cargo testの時だけ、このん部分をコンパイル
・非公開関数もテストok

3️⃣結合テストのルール　→ 完成品を外から使ってみるテスト
・testsディレクトリを作る。srcフォルダの隣に、testsフォルダを作る。ここでは#[cfg(test)]を書く必要はない
・公開しか使えない

4️⃣testsフォルダの中に、複数のテストで使いまわしたい共通の準備関数をまとめたcommon.rsを作ったとする。
そのままcargo testをするとだめ
tests/common/mod.rsとする

5️⃣重要なロジックは全部src/lib.rsに書いて、src/main.rsはそれを呼び出すだけのスカスカな状態
にしておくというテクニックがある。
*/

use test_systematization::get_task_status;

fn main(){
    let days = 3;

    let status = get_task_status(days);

    println!("現在の課題の状況:{}",status);
}
