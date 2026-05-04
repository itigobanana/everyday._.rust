//スライスは参照の一種！！

fn main(){
    // no1();
    no2();
    no3();
    no4();
    no5();
}

//ここよくわかってない。スライスの導入だからなんとなくでもいい？？
// fn no1() {
//     let mut s = String::from("hello world");
//     let _word = first_word(&s);

//     s.clear();
    
// }

// fn first_word(s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }


fn no2(){
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    let _slice = &s[3..];
    let _slice = &s[..2];
}


fn no3(){
    let hello = String::from("hello");
    first_word(&hello);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..] //もしループの中で空白が見つからなかったら
}



//関数の引数は&strにしたほうが楽だよっていう例！
fn no4(){
    let my_string = String::from ("hello world");

    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(my_string_literal);
}

fn no5(){
    let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
}