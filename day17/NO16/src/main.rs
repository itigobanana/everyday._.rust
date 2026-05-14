fn main() {
    let mut mileage = 170000;

    drive(&mut mileage);
    println!("ドライブ後の走行距離:{}km",mileage);
}

fn drive(m: &mut i32){
    *m += 50;
}