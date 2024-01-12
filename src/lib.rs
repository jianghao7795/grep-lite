// use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use std::io::Read;

mod add_two;
mod err_handle;
mod iterator;
mod json_serde;
pub mod string_slice;
mod trait_demo;

pub fn run_trait() {
    let a = trait_demo::trait_add::add(1, 2);
    println!("{a}");
    let b = trait_demo::trait_add::add(1.1, 3.3);
    println!("{b}");

    trait_demo::trait_add::demo();
}

pub fn run_json() {
    let my_data = json_serde::json_serde_data::json_to_data().expect("str to json fail");
    println!("{:?}", my_data);
}

pub fn run_err_handle() {
    err_handle::err_handle::err_result();
}

pub fn iterat() {
    iterator::iterator::iterat();

    iterator::iterator::point();

    iterator::iterator::matching();
}

error_chain! {
   foreign_links {
       Io(std::io::Error);
       Decode(data_encoding::DecodeError);
   }
}

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn sheet() {
    println!("hello, world, {}", add_two::add_two::add_two(2));
}

pub fn get_app() -> clap::App<'static, 'static> {
    clap::App::new("grep-lite")
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
                .help("The input file to use")
                .takes_value(true)
                .required(true),
        )
}

#[cfg(test)]
pub mod tests {
    use crate::get_app;
    use clap::ErrorKind;

    #[test]
    pub fn test_no_args() {
        let res = get_app().get_matches_from_safe(vec!["bbbbgrep4444lite1111"]);
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
    }
}
