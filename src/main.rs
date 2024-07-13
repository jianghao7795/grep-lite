extern crate clap; // extern crate 链接一个外部包,或者一个宏变量(该变量定义在另外一个包中)

use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        // match re.find(&line) {
        //    Some(_) => println!("{}", line),
        //    None => (),
        //}
        if re.find(&line).is_some() {
            println!("{}", line);
        }
    }
}

fn main() {
    let args = clap::App::new("grep") // 应用名
        .version("0.1") // 应用version
        .about("searches for patterns") // about
        .author("user") // user
        .arg_from_usage("-p, --path=[FILE] 'Target file you want to change'") // Options 输入错误报错
        .arg(
            // 输入字段
            clap::Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            // 输入的文件名
            clap::Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re)
    }
}
