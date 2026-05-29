use std::env;//引数を受け取るためのテンプレ
use std::fs;//ファイル操作のためのモジュールを追加
use io_project::Config;
use std::process;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("入力エラー：{}",err);//ルートを分ける。失敗の時は画面に表示、成功の時はoutput.txtに保存
        process::exit(1);
    });

    println!("探している言葉：{}",config.query);
    println!("対象のファイル：{}",config.file_path);

    if let Err(e) = io_project::run(config){
        eprintln!("アプリ実行中にエラーが発生しました:{}",e);
        process::exit(1);
    }
}

//env::args()....ターミナルに入力され得た文字を順番に読み取る（イテレータ）
//.collect()...ひとまとめのコレクションにする
/*
cargo new needle hay.txt->
出力["/Users/yuitaira/Study/rust/day21/part12/io_project/target/debug/accept_commandline", 
"needle", "hay.txt"]
[パス、一つ目の引数（検索したい言葉）、二つ目の引数（ファイル名）]*/

