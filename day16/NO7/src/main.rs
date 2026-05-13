struct HairStatus{
    length: String,
    color: String,
    bleach_count: i32,
}

fn main(){
    let my_hair = HairStatus{
        length: String::from("ロング"),
        color: String::from("茶色"),
        bleach_count: 3,
    };

    println!("今の髪は{}。{}回ブリーチ済み！",my_hair.color,my_hair.bleach_count);
}