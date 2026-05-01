use std::io;

fn main() {
    //     let gess: i32 = "42".parse().expect("Not a numbet!");

    //i(符号付き -&+),u(符号なし 0<=n),大体i32でいい
    //小数はf64

    // let tup = (500, 6.4, 1);
    // let (x,y,z) = tup;
    // println!("The value of y is {y}!")

    //タプルはさまざまな型の複数の値をまとめる。長さは固定。
    // let x: (i32,f64,u8) = (500,6.4,1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // println!("{}",five_hundred);

    //配列は型固定。長さは固定。
    //要素数を変える必要はないときは配列がいいね！！
    // let a: [i32; 5] = [1,2,3,4,5]; //i32の方の要素が5こ
    // let a = [3; 5]; //3が5こ == [3,3,3,3,3]

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
