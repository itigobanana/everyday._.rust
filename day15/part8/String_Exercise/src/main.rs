/*文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。
ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。 
UTF-8エンコードに関する詳細を心に留めておいてください！*/

fn main() {
    let s = String::from("first");

    println!("{}",big_latin(&s));
}

fn big_latin(word: &str) -> String{
    let first_char = match word.chars().next(){
        Some(c) => c,
        None => return String::new(),
    };
    
    match first_char{
        'a'|'i'|'u'|'e'|'o'|'A'|'I'|'U'|'E'|'O' =>{
            format!("{}-hay", word)
        }
        _ => {
            let first_char_len = first_char.len_utf8();
            let rest_of_word = &word[first_char_len..];

            format!("{}-{}ay",rest_of_word,first_char)
        }
    }
}