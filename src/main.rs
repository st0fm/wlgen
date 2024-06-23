use std::io::{self, BufRead};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'n', long, default_value = "1")]
    num: u8,

    #[arg(short = 's', long, default_value = "/")]
    seperator: String,

    #[arg(short = 'p', long, default_value = "")]
    prefix: String,

    #[arg(short = 'P', long, default_value = "")]
    postfix: String,
}

fn generate_wordlist(word: &str, wordlist: &Vec<String>, seperator: &str) -> Vec<String> {
    wordlist
        .into_iter()
        .map(|line| {
            format!(
                "{line}{seperator}{word}",
                line = line,
                seperator = seperator,
                word = word
            )
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    let words: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut result: Vec<String> = words.clone();

    for _ in 0..args.num {
        let mut tmp_result: Vec<String> = Vec::new();
        for word in &words {
            tmp_result.append(&mut generate_wordlist(word, &result, &args.seperator));
        }
        result = tmp_result;
    }

    let result: Vec<String> = result
        .into_iter()
        .map(|line| {
            format!(
                "{prefix}{line}{postfix}",
                prefix = &args.prefix,
                line = line,
                postfix = &args.postfix
            )
        })
        .collect();

    for word in &result {
        println!("{}", word);
    }
}
