use std::io::{self, BufRead};

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'k', long, default_value = "1")]
    max: usize,

    #[arg(short = 'm', long)]
    min: Option<usize>,

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
    let min = args.min.unwrap_or(args.max);

    let words: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    for k in min..=args.max {
        for elements in words.clone().into_iter().permutations(k) {
            let line: String = if args.upper {
                elements
                    .into_iter()
                    .filter(|e| !e.is_empty())
                    .map(|e| e[0..1].to_uppercase() + &e[1..])
                    .collect::<Vec<_>>()
                    .join(&args.seperator)
            } else {
                elements.join(&args.seperator)
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
