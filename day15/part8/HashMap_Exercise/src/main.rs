/*ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられる
テキストインターフェイスを作ってください。 
例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や
"Add Amir to Sales"(販売部門にアミールを追加)などです。 
それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で
並べ替えられた会社の全人間の一覧を扱わせてあげてください。*/

use std::collections::HashMap;
use std::io;

fn main(){
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    println!("会社システムを起動しました！");

    loop{
        println!("________________________________________________________");
        println!("コマンドを入力してください。");
        println!("\nコマンドを入れてね");
        println!("追加：Add [名前] to [部署]");
        println!("部署一覧：List [部署]");
        println!("全社一覧：List All");
        println!("終了：Quit");
        print!("> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("読み込みエラーです");

        let words: Vec<&str> = input.trim().split_whitespace().collect();

        if words.is_empty(){
            continue;
        }

        match words[0]{
            "Add" => {
                if words.len() == 4 && words[2] == "to"{
                    let name = words[1].to_string();
                    let department = words[3].to_string();
                    company.entry(department.clone()).or_insert(Vec::new()).push(name.clone());
                    println!("{} を {}に追加しました。",name, department);
                    }else{
                        println!("入力形式が違います。'Add 名前 to 部署名'で入れてください");
                    }
            }
            "List" => {
                if words.len() == 2{
                    if words[1] == "All"{
                        let mut depts: Vec<&String> = company.keys().collect();
                        depts.sort();

                        println!("________________________会社の名簿___________________________");
                        for dept in depts{
                            println!("[{}部署]", dept);

                            let mut names = company[dept].clone();
                            names.sort();

                            for name in names{
                                println!(" - {}",name);
                            }
                        }
                    }else{
                        let department = words[1];

                        if let Some(names) = company.get(department){
                            let mut sorted_names = names.clone();
                            sorted_names.sort();

                            println!("_______________________{}部署の人たち_________________________",department);
                            for name in sorted_names{
                                println!(" - {}", name);
                            }
                        }
                    }
            } else{
                println!("フォpマットが違います。書き直してください");
            }
        }
            "Quit" | "Exit" => {
                println!("システムを終了します");
                break;
            }
            _ => {
                println!("そのコマンドには対応していません");
            }
        }
    }
}
