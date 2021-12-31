use rand::prelude::*;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    fn need_input() -> String {
        let mut word = String::new();
        io::stdin().read_line(&mut word).ok();
        return word.trim().to_string();
    }

    println!("契約書だよ。そこに名前を書きな");

    let origin_name = need_input();

    println!(
        "フン。{name}というのかい。贅沢な名だねぇ。",
        name = origin_name
    );

    let name_len = origin_name.trim().chars().count();

    let num = rng.gen_range(0, name_len - 1);

    let new_name = origin_name.chars().nth(num).unwrap();

    println!(
        "今からお前の名前は{name}だ。いいかい、{name}だよ。分かったら返事をするんだ、{name}!!",
        name = new_name
    );
}
