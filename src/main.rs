use data_encoding::HEXUPPER;
use grep_lite::sha256_digest;
use grep_lite::sheet;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops;
use std::ops::Add;
use std::thread as threaded;

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
mod sortor;

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
        println!("{}", 1);
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

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}

impl Add for Meters {
    type Output = Self;
    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
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

    println!("{}", 90);
    println!("{}", "wode");
    println!("{}", 6665555);
    println!("{}", "strings");
    println!("{}", String::from("new String"));
    println!("{}", 'a');

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("{}", w);

    let d = calculate_distance(Meters(50), Meters(20));
    println!("{}", d);
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

    // let mut output = match File::create(path) { // 只能把文件清空 重新写入
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };
    // // write!(output, "We will generate a digest of this text, me")?;
    // output.write(b"buf").unwrap();

    let mut output = OpenOptions::new() // 可以追加写入
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
    let write_number = output
        .write(b"We will generate a digest of this text, me.\n")
        .unwrap();
    println!("{}", write_number);

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader).unwrap();

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    let ordinary_string = "ordinary string".to_string();
    let host = Hostname(ordinary_string.clone(), 23);
    connect(host);

    // let value = match returns_ok() {
    //     Ok(value) => value,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // println!("Got value: {:?}", value);
    // let value = returns_err();
    // println!("{:?}", value);
    // Open file 对返回的错误进行处理
    // let f = File::open("./hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(ref error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    // f.write("buf".as_bytes())?;
    // 打开文件
    let f1 = read_username_from_file()?;
    println!("{}", f1);
    // 读取文件
    // open_file()?;
    let f2 = open_file();
    let mut f3 = match f2 {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut buf = String::new();
    println!("182: {:?}", f3.read_to_string(&mut buf)?);
    println!("183: {:?}", buf);

    let mut s = String::new();
    let mut update_string = |str| s.push_str(str); // 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型
    update_string("Hello, ");
    println!("{:?}", s);

    let v = vec![1, 2, 3];
    // move 闭包会获取其使用的环境值的所有权 后面不能再使用
    let handle = threaded::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // handle.join().unwrap();// 错误 因为所有权已经转移
    // println!("{:?}", v);
    // exec();
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());

    let mark_twain = "I have never let my schooling interfere with my education.";
    print_author(&mark_twain);
    {
        let a = 5;
        println!("{a}");
        println!("{a}");
        let c = 8;
        println!("{c}");

        let v = vec![1, 2, 3];
        println!("{}", v[0]);
        println!("{}", 5);
    }
    {
        equal::equal_one::rand();
        equal::equal_one::rand_num();
        let _ = equal::equal_two::normal();
    }
    {
        sortor::sortor::sortor1();
        sortor::sortor::sortor2();

        sortor::random::point();

        grep_lite::iterat();
    }
    Ok(())
}

fn print_author<T: fmt::Display>(str: &T) {
    println!("Mark Twain {}", str);
}

//FnOnce 闭包只能被调用一次
// func 的类型 F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
    println!("{}", func(5));
}

#[allow(dead_code)]
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(f)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? 代替 match ?是个宏
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Result<T, E> 代表可能出错的操作
// fn returns_ok() -> Result<String, MyError> {
//     Ok("everything is fine".to_string())
// }

// fn returns_err() -> Result<String, MyError> {
//     Err(MyError("something went wrong".to_string()))
// }

#[derive(Debug)]
struct MyError(String);
// 元组结构体(Tuple Struct)
struct Hostname(String, i32);

fn connect(host: Hostname) {
    println!("Connecting to {}", host.0);
    println!("Connecting to i32 {}", host.1);
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
