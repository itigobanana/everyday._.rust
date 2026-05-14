struct App{
    name: String,
}

impl App{
    fn start(&self){
        println!("{}を起動します",self.name);
    }
}

fn main(){
    let my_app = App{name: String::from("アプリ")};
    my_app.start();
}