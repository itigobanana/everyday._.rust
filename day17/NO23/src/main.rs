fn main() {
    let sales = vec![
        Some(1200),
        None,
        Some(950),
        Some(1500),
        None,
    ];

    let mut total_sales = 0;

    for item in sales{
        if let Some(price) = item{
            total_sales = total_sales + price;
        }
    }

    println!("現在の売り上げ合計は{}円です。",total_sales);
}
