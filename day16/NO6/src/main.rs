fn main() {
    let price: i32 = 175;
    let liters: f64 = 25.0;

    let total = price as f64 * liters;

    println!("満タンにするには{}円かかるよ",total);
}
