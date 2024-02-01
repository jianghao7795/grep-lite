// use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use std::io::Read;

mod add_two;
mod candle;
mod closure_fn;
mod err_handle;
mod expression;
mod fmt_display;
mod generality_feature;
mod if_cfg;
mod iterator;
mod json_serde;
mod linked;
mod linked_list;
mod mutable;
mod phantom_type;
mod point;
mod raii;
mod string_slice;
mod trait_demo;
mod tuple_string;

pub fn run_linked() {
    let mut list = linked::List::new();

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    println!("list pop is {:?}", list.pop());
    println!("list pop is {:?}", list.pop());
    println!("list head is {:?}", list.peek());
    list.peek_mut().map(|value| *value = 17);
    println!("list mut head is {:?}", list.peek());

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    // assert_eq!(list.pop(), Some(5));
    // assert_eq!(list.pop(), Some(4));
    println!("list pop is {:?}", list.pop());
    println!("list pop is {:?}", list.pop());

    // Check exhaustion
    // assert_eq!(list.pop(), Some(1));
    // assert_eq!(list.pop(), None);
    println!("list pop is {:?}", list.pop());
    println!("list pop is {:?}", list.pop());
}

pub fn run_trai_demo() {
    trait_demo::animal::demo();
    println!("{:?}", "How you doing".split(' ').collect::<Vec<_>>());
}

pub fn run_phantom() {
    phantom_type::run_phantom();
}

pub fn run_candle() {
    let _ = candle::candle_explmt().expect("error");
}

pub fn run_if_cfg() {
    if_cfg::run_cfg();
}

pub fn run_point() {
    point::destructure();
}

pub fn run_closure_fn() {
    closure_fn::run_closure();
}

pub fn run_expression() {
    expression::run_expression();
}

pub fn run_fmt_display() {
    fmt_display::run_fmt();
}

pub fn run_mutable() {
    mutable::run_book();
    mutable::run_point();
}

pub fn run_raii() {
    raii::run_box();
}

pub fn run_linked_list() {
    linked_list::linked::run_linked();
    linked_list::linked::run_ref();
}

pub fn run_string_slice() {
    string_slice::string_slice::slice_example();
    string_slice::string_slice::string_str();
    string_slice::slice::run_slice();
}

pub fn run_tuple_string() {
    tuple_string::tuple::run_reverse();
}

pub fn run_generality_feature() {
    generality_feature::generality::run_add();
    generality_feature::generality::run_struct_add_sub();

    generality_feature::generality::run_mixup();

    generality_feature::generality::run_display_array();

    let t = generality_feature::feature::returns_summarizable(false);
    println!("{:?}", t.summarize());
}

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

    closure_fn::input();
    iterator::iterator::demo();
    println!("{:?}", "How you doing".split(' ').collect::<Vec<_>>());
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
