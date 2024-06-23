use std::io::{self, BufRead};

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'm', long, default_value = "1")]
    min: usize,

    #[arg(short = 'M', long, default_value = "2")]
    max: usize,

    #[arg(short = 'u', long, default_value = "false")]
    upper: bool,

    #[arg(short = 's', long, default_value = "")]
    seperator: String,

    #[arg(short = 'p', long, default_value = "")]
    prefix: String,

    #[arg(short = 'P', long, default_value = "")]
    postfix: String,
}

fn main() {
    let args = Args::parse();

    let words: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    for k in args.min..=args.max {
        for elements in words.iter().permutations(k) {
            let line: String = if args.upper {
                elements
                    .iter()
                    .map(|e| e[0..1].to_uppercase() + &e[1..])
                    .collect::<Vec<_>>()
                    .join(&args.seperator)
            } else {
                elements.iter().join(&args.seperator)
            };

            println!(
                "{prefix}{line}{postfix}",
                prefix = &args.prefix,
                line = line,
                postfix = &args.postfix
            );
        }
    }
}
