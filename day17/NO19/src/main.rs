fn main() {
    let config = Some("ダークモード");

    if let Some(mode) = config{
        println!("アプリの見た目を{}に変更しました",mode);
    }else{
        println!("設定が見つからないよ");
    }
}
