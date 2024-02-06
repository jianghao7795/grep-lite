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
mod intoiter;
mod iterator;
mod json_serde;
mod linked;
mod mutable;
mod phantom_type;
mod point;
mod raii;
mod rc;
mod string_slice;
mod thread;
mod trait_demo;
mod tuple_string;

pub fn run_thread() {
    thread::signed::signed_demo();
    thread::signed::signed_double();
    thread::signed::mutex();

    thread::signed::thread_arc();
}

pub fn run_linked_third() {
    let list = linked::third::List::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
    let list = linked::third::List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

pub fn run_linked_second() {
    let mut list = linked::second::List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    let mut list = linked::second::List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| *value = 42);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
    let mut list = linked::second::List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);

    let mut list = linked::second::List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));

    let mut list = linked::second::List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}

pub fn run_linked_first() {
    let mut list = linked::first::List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

pub fn run_rc() {
    rc::run_rc();
}

pub fn run_trai_demo() {
    trait_demo::animal::demo();
    println!("{:?}", "How you doing".split(' ').collect::<Vec<_>>());
}

pub fn run_phantom() {
    phantom_type::run_phantom();
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
