use axum::{routing::get, Router};

//#[tokio::main]...asyncを動かすためのマクロ
#[tokio::main]

//async（非同期）...Webサーバーは同時に何人もアクセスしてくるので、爆速マルチタスクを可能にするためにつける
async fn main(){

    //"/"に人が来たら、hello_world関数を呼ぶ。getは見るだけ。postとかだとデータを送信！
    let app = Router::new().route("/", get(hello_world));

    //.await.unwrap()...ドアの鍵を開ける通信作業が終わるまでちょっと待って、
    //  もし他のアプリがすでに3000番ドアを使ってたらpanicして！
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("サーバー起動！http://127.0.0.1:3000 にアクセスしてみて！");

    //listenerとappを合体させて、営業中にする。
    //.awaitにより、強制終了されるまでずっと待ち続ける無限ループ。
    axum::serve(listener, app).await.unwrap();
}

//"&'static str"...静的領域に書いてある文字のリンクをブラウザにペタッと貼り付けて返す
async fn hello_world() -> &'static str{
    "Hello, Rust Backend World!!!"
}