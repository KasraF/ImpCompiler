extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::pest::Parser;
use std::error::Error;

const PROGRAM: &'static str = "if true then 1 else 2";

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ImpParser {}

fn main() -> Result<(), &'static dyn Error> {
    println!("{:?}", ImpParser::parse(Rule::expr, PROGRAM));
    Ok(())
}
