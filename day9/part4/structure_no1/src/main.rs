//一部のフィールドのみを可変にすることはできない！
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

#[derive(Debug)]
struct human{
    name:String,
    height: usize,
    weight: usize,
}

struct rabbit{
    name: String,
    favorite_grass: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //[..]で、残りのフィールドについてはuser1と同じ値になるインスタンスを生成する
    let user2 = User{
        email: String::from("another@example.com"),
        ..user2
    };

    let azuki = rabbit{
        name: String::from("azuki"),
        favorite_grass: String::from("kusa"),
    };

    let ibuki = human{
        name: String::from("ibukisinzato"),
        height: 100,
        weight: 500,
    };

    println!("{:?}",ibuki);



}

// fn build_user(email: String, username: String) -> User{
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }


//空の構造体は、中身のデータは必要ないけど、型の名前だけ欲しいって時に使うらしい
//あんまり使わないらしい！！！

struct AlwaysEqual;

// fn main(){
//     let subject = AlwaysEqual;
// }


//＆strじゃなくて、String型にしましょうねー、とっても難しい！
// struct User{
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main(){
//     let user1 = User{
//         active = true,
//         username: "someusername123"
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }