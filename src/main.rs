#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use pest::Parser;

// Workaround for Rust 2018 preview, "cannot find attribute macro `grammar` in this scope"
#[macro_use]
extern crate pest_derive;

// https://crates.io/crates/pest crates.io latest => 1.0.6
// https://github.com/pest-parser/pest/blob/v1.0.6/pest_derive/src/lib.rs#L263
// https://doc.rust-lang.org/book/first-edition/procedural-macros.html
// https://rust-lang-nursery.github.io/edition-guide/2018/transitioning/modules/macros.html
//use pest_derive::derive_parser; // error[E0432]: unresolved import `pest_derive::derive_parser`, no `derive_parser` in the root



const _GRAMMAR: &str = include_str!("csv.pest");

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    println!("{:?}", unsuccessful_parse);
}
