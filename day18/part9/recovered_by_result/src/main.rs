// fn main() {
//     let denchu = Pokemon {
//         name: "Denchu".to_string(),
//         age: 34,
//     };

//     denchu.attack("凛ちゃん".to_string());
// }

// struct Pokemon {
//     name: String,
//     age: usize,
// }

// impl Pokemon {
//     fn attack(&self, aite: String) {
//         println!("{}に10まんボルト！100ダメージ！", aite);
//     } 
// }


/*
ファイルを開くなどの処理をしたとき、ファイルの中身を直接渡すわけではなく、
resutlという名前の箱を渡してくる
resultの中身はOkかErr
*/

/* 

//ファイルがあるかないかわかんないとき
use std::fs::File;

fn main(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result{
        Ok(file) =>{
            println!("ファイルが無事に開けたよ");
            file
        },
        Err(error) => {
            panic!("ファイルを開けないよー！！:{:?}",error);
        },
    };
}

//ファイルが絶対あるとき

fn main(){
    let greeting_file = File::open("hello.txt").expect("hello.txtは絶対に存在するはずなのに！");
    let greeting_file = File::open("hello.txt").unwrap();
}

*/


//エラーが起きた関数にエラーを丸投げ
//fn mainは基本関数を返さないのでいい感じにする
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {

    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    println!("ファイルの中身は：{}",username);

    Ok(())
}