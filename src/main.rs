#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub iqparser); 
mod ast;

fn main() {
    
    assert!(iqparser::SliceExprParser::new().parse("+").is_ok());

}



