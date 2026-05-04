//任意のタイミングで、一つの可変参照か不変な参照いくつでものどちらかを行える。
//参照は常に有効でなければならない。

fn main() {
    println!("\n");
    no1();
    println!("\n");
    no2();
    println!("\n");
    no3();
    println!("\n");
}


//&をつけることによって、所有権をもらうことなく値を参照する。
//&でデータを参照している間は、データの中身を変えることができない
fn no1(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.",s1,len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}



//可変参照は一つまで。同じデータへの複数の可変参照が同時に存在することは禁止！
//不変な参照をしている間は、同じ値に対して可変な参照をすることはできない。

fn no2(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);
}

fn change(some_string: &mut String){
    some_string.push_str(",world!");
}


//不変と可変は共存できないよっていう例
//使われてない変数は死んでるって判断してくれるらしい
fn no3(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}",r1,r2);

    let r3 = &mut s;
    println!("{}",r3);
    //ここでr1とr2を呼び出すと、エラー
}