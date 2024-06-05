use std::env;
use unicode_width::UnicodeWidthChar;
fn main() {
    let file = std::fs::read_to_string("./AA.txt");
    let aa = match  file{
        Ok (_) =>file.unwrap(),
        Err(_) => "　　　　　　　　　　　　　 ,,,z=~'ﾞ'+''ｯ彡ｯ,､
        　　　　　　　　　　　　,ｨ´ 　　　　　 \"':';:;ｯ;,
        　　　　　　　　 ,　' ﾞ´`ﾞﾐﾞｯ,　　　　　　　 \"',`,
        　　　　　　 ,／ 　　　 `､ﾞミ　　　　　　　　 ﾞ:;:,
        　　　　　 /　　　　　 _ =ヾ､ﾞｼｼ=;,z,、　　　 ﾞ;ｼ::ﾐ
        　　　　 /　　　　 ,ｒ,´　　 /　´`ヽ ゛ﾞ`　 　　,ﾞ彡:ﾐ
        　　　 / 　　　, '-､_`ヽ_/,　　　　　　　 　 ﾐ;::彡;:
        　　 ,'　　　,ｼ´｀｀ ヽ`i｀!　　　　　　　　 ,,彡;::ｼ:彡　
        　　;i　　､（´ ￣`ヽ / '　　　　　　　　シ:ｼ;:ﾐ::ｼ\"
        　ノ:!､　 ヽ｀`ｰ =;ｨ'　　　　　　　　,,ｼ:;彡;ｼﾞ
        ´:::::.ヾ. 　　　￣´　　　　　　　　' `,ｼﾐﾞ
        :::::::::::::.`:ヽ､_　　　　　　　...:;'＿,ソ'ﾞ''
        ::::::::::::::::::::::::::｀:::::::::::::::-='".to_string(),
    };
    let mut word:String ="".to_string();
    if env::args().len() == 1 {
        word = "イキスギィ".to_string();
    } else {
        for (index,_element) in env::args().enumerate() {
            if index!=0 {
                word.push_str(&env::args().nth(index).unwrap());
                if index != env::args().len()-1 {
                    word.push_str(" ");
                }
            }
        }
    }
    print!(" ");
    for char in word.chars() {
        if UnicodeWidthChar::width(char) == Some(2) {
            print!("＝");
            continue;
        }
        print!("=");
    }
    println!(" ");
    print!("<");
    print!("{word}");
    println!(">");
    print!(" ");
    for char in word.chars() {
        if UnicodeWidthChar::width(char) == Some(2) {
            print!("＝");
            continue;
        }
        print!("=");
    }
    println!(" ");
    println!("{}",aa);
}
