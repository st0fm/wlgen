use std::io::{self, BufRead};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'n', long, default_value = "1")]
    num: u8,

    #[arg(short = 'm', long)]
    min: Option<u8>,

    #[arg(short = 's', long, default_value = "/")]
    seperator: String,

    #[arg(short = 'p', long, default_value = "")]
    prefix: String,

    #[arg(short = 'P', long, default_value = "")]
    postfix: String,
}

fn generate_wordlist(word: &str, wordlist: &[String], seperator: &str) -> Vec<String> {
    wordlist
        .iter()
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
    let min = args.min.unwrap_or(args.num);

    let words: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut tmp_result: Vec<String> = words.clone();
    let mut result: Vec<String> = Vec::new();

    for round in 1..args.num {
        let mut tmp: Vec<String> = Vec::new();

        if round >= min {
            result.extend(tmp_result.clone());
        }

        for word in &words {
            tmp.extend(generate_wordlist(word, &tmp_result, &args.seperator));
        }
        tmp_result = tmp;
    }

    result.append(&mut tmp_result);

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
