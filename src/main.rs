#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use pest::Parser;

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
