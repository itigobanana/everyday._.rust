enum Language{
    Python,
    Cpp,
    Rust,
}

fn main(){
    let my_lang = Language::Rust;

    match my_lang{
        Language::Python => println!("{}",1),
        Language::Cpp => println!("{}",2),
        Language::Rust => println!("{}",3),
     }
}