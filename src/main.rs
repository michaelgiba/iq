use crate::ast::IqAstRootNode;
use crate::context::BasicContext;
use crate::eval::Evalulate;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub iqparser);
mod ast;
mod attrs;
mod context;
mod eval;
mod ops;

fn main() {
    let root: IqAstRootNode = iqparser::IqRootParser::new()
        .parse(
            "
            _ => p(_.y, _.x, _.r, (_.x / [].w) * 255, _.b)
        ",
        )
        .unwrap();

    let input_context = BasicContext::from_path("in.jpg");
    let context = root.eval(&input_context);
    context.write("out.jpg");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_passes() {
        let parser = iqparser::IqRootParser::new();
        assert!(parser.parse("").is_ok());
        assert!(parser.parse("[]").is_ok());
        assert!(parser.parse("~").is_ok());

        assert!(parser
            .parse(
                "
            sqrt(sq(_.y - [].c.y) - sq(_.x - [].c.x)) <= 1 => _;            
        "
            )
            .is_ok());

        assert!(parser
            .parse(
                "
            [] | sqrt(sq(_.y - [].c.y) - sq(_.x - [].c.x)) <= 1 => _       
        "
            )
            .is_ok());

        assert!(parser
            .parse(
                "
            [] | sqrt(sq(_.y - [].c.y) - sq(_.x - [].c.x)) <= 1 => _;            
            [10:20,30:50] | sqrt(sq(_.y - [].c.y) - sq(_.x - [].c.x)) <= 1 => _;            
        "
            )
            .is_ok());

        assert!(parser
            .parse(
                "
            [:[].h/2] | _ => p(_.y, _.x, _.r, 0, 0)       
        "
            )
            .is_ok());
    }

    #[test]
    fn parser_fails() {
        let parser = iqparser::IqRootParser::new();
        assert!(!parser.parse("|").is_ok());
    }
}
