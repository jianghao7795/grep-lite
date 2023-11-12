use data_encoding::HEXUPPER;
use grep_lite::sha256_digest;
use grep_lite::sheet;
// use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

mod add_five;
mod add_four; // 代表文件和文件夹
mod add_three; // 代表文件夹 add_three
mod add_two; // 代表文件 add_two.rs
mod equal;

mod add {
    pub mod add_noe {
        pub fn add_one(base: u32) -> u32 {
            base + 1
        }
    }
}

struct Foo;
struct Bar;
#[derive(PartialEq, Debug)]
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;

// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}
// 下面的代码实现了自定义类型的相减： Bar - Foo = BarFoo
impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = clap::App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            clap::Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
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
    println!("{}", crate::add::add_noe::add_one(55));
    println!("{}", add_two::add_two::add_two(77));
    println!("{}", add_three::add_three::add_three(99));
    println!("{}", add_four::add_four::add_four(50));
    println!("{}", equal::equal_one::equal_one(4));
    println!("{}", add_five::add_five::add_five_then_equal_one(0));

    println!("{}", add_five::add_five::add_six::add_six(55));
    println!("{}", add_five::add_five::add_six::delete_six(55));

    sheet();

    let mut a = String::from("test");
    println!("{a}");
    a = String::from("24234234");
    println!("{a}");

    let p = Point {
        x: 5.0f32,
        y: 10.0f32,
    };
    println!("{}", p.distance_from_origin());

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Bar - Foo, BarFoo);

    println!("Success!");

    // let path = "file.txt";
    // let mut output = File::create(path)?;
    // write!(output, "We will generate a digest of this text")?;

    // let input = File::open(path)?;
    // let reader = BufReader::new(input);
    // let digest = sha256_digest(reader)?;

    // println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    // // sha256_digest(reader)
    // Ok(())
    let path = "file.txt";

    let mut output = match File::create(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader).unwrap();

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
