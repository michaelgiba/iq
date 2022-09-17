use crate::ast::IqAstRootNode;
use crate::eval::Evalulate;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub iqparser);

#[allow(clippy::large_enum_variant)]
mod ast;
mod attrs;
pub mod context;
mod ctx_ops;
mod eval;
mod float_ops;

pub fn execute(input_ctx: context::BasicContext, expressions: String) -> context::BasicContext {
    let root: IqAstRootNode = iqparser::IqRootParser::new()
        .parse(expressions.as_str())
        .unwrap();

    root.eval(&input_ctx)
}
