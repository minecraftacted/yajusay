use std::env;
use unicode_width::UnicodeWidthChar;
fn main() {
    let yjsnpiaa = "　　　　　　　　　　　　　 ,,,z=~'ﾞ'+''ｯ彡ｯ,､
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
    ::::::::::::::::::::::::::｀:::::::::::::::-='";
    let mut word:String;
    if env::args().len() == 1 {
        word = "イキスギィ".to_string();
    } else {
        word = env::args().nth(1).unwrap();
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
    println!("{}",yjsnpiaa);
}
