//ジェネリクス...型を後から決めるための空箱（プレースホルダ）


fn main(){

//<T>という何かしらの型。このコードだとエラーになる
fn largest<T>(list: &[T]) -> &T {}

//この場合はxもyも同じ型でないといけない
struct Point<T> {
    x: T,
    y: T,
}

//同じ型じゃなくてもオッケー
struct Point<T, U> {
    x: T,
    y: U,
}

//Option<T>　→　Some(T) or None
//Result<T, E> →　Ok(T) or Err(E)


//impl<T> で、今からジェネリクスTを使って実装するよ、というRustへの宣言
impl<T> Point<T> {
    fn x(&self) -> &T {}
}

//f32の時だけ使える特別なメソッド
impl Point<f32> {
    fn distance_from_origin(&self) -> f32{}
}

}

//ジェネリクス<T>は、関数、構造体などを、型にとらわれない柔軟な姿にアップデートできる！