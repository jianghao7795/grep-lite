use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

mod add_four; // 代表文件和文件夹
mod add_three; // 代表文件夹 add_three
mod add_two; // 代表文件 add_two.rs

mod add {
    pub mod add_noe {
        pub fn add_one(base: u32) -> u32 {
            base + 1
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    // let f = File::open(input).unwrap();
    // let reader = BufReader::new(f);

    // for line in quote.lines() {
    //     match re.find(line) {
    //         Some(_) => println!("{}", line),
    //         None => (),
    //     }
    // }

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re)
    }

    println!("{}", crate::add::add_noe::add_one(55));
    println!("{}", add_two::add_two::add_two(77));
    println!("{}", add_three::add_three::add_three(99));
    println!("{}", add_four::add_four::add_four(50));
}
