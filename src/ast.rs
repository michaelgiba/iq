use std::fmt::{Debug, Error};
use std::option::Option;

pub struct AttrAccessNode {
    key: char,
}

pub struct SelectorScalarNode {
    pub select_ctx: SelectorCtxNode,
    pub accessed_attr: AttrAccessNode,
}

pub enum ScalarNode {
    Number(f64),
    SelectorScalar(SelectorScalarNode),
    CurrentPixelScalar(AttrAccessNode),
}

pub enum BinaryOpType {
    Add(),
    Subtract(),
    Divide(),
    Multiply(),
}

pub enum MatchOpType {
    Lt(),
    Lte(),
    Gt(),
    Gte(),
    Eq(),
    Neq(),
}

pub struct ColorNode {
    pub r_expr: ScalarExprNode,
    pub g_expr: ScalarExprNode,
    pub b_expr: ScalarExprNode,
}

pub struct MatchComparatorNode {
    pub op_type: MatchOpType,
    pub comp_val: ColorNode,
}

pub struct PixelMatchExprOpNode {
    pub match_comparator_node: MatchComparatorNode,
    pub match_return_value_node: ColorNode,
}

pub enum OperatorNode {
    UnaryNegationOp(),
    PixelMatchExprOp(PixelMatchExprOpNode),
}

pub struct BinaryScalarOpNode {
    pub lhs: ScalarExprNode,
    pub op: BinaryOpType,
    pub rhs: ScalarExprNode,
}

pub enum ScalarExprNode {
    SubExpr(Box<ScalarExprNode>),
    Scalar(ScalarNode),
    BinaryOp(Box<BinaryScalarOpNode>),
}

pub struct SliceRangeNode {
    pub lower_bound: Option<ScalarExprNode>,
    pub upper_bound: Option<ScalarExprNode>,
}

pub struct SelectorCtxNode {
    pub y_slice_range: Option<Box<SliceRangeNode>>,
    pub x_slice_range: Option<Box<SliceRangeNode>>,
}

pub struct ExprNode<'a> {
    pub selector_ctx: Option<SelectorCtxNode>,
    pub op_nodes: &'a Vec<OperatorNode>,
}

pub struct IqRootNode<'a> {
    pub exprs: &'a Vec<ExprNode<'a>>,
}
