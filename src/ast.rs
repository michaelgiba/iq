use std::fmt::Debug;
use std::option::Option;

#[derive(Debug, Clone)]
pub struct AttrAccessNode {
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct SelectorScalarNode {
    pub selector_ctx: SelectorCtxNode,
    pub accessed_attr: AttrAccessNode,
}

#[derive(Debug, Clone)]
pub enum ScalarNode {
    Float(f64),
    Integer(i64),
    SelectorScalar(SelectorScalarNode),
    PixelScalar(Box<PixelExprType>, AttrAccessNode),
}

#[derive(Debug, Clone)]
pub enum BinaryOpType {
    Add(),
    Sub(),
    Div(),
    Mul(),
}

#[derive(Debug, Clone)]
pub enum MatchOpType {
    Lt(),
    Lte(),
    Gt(),
    Gte(),
    Eq(),
    Neq(),
}

#[derive(Debug, Clone)]
pub struct PixelNode {
    pub y_expr: ScalarExprNode,
    pub x_expr: ScalarExprNode,
    pub r_expr: ScalarExprNode,
    pub g_expr: ScalarExprNode,
    pub b_expr: ScalarExprNode,
    pub a_expr: ScalarExprNode,
}

#[derive(Debug, Clone)]
pub enum PixelExprType {
    Explicit(PixelNode),
    CurrentPixel(),
    FnCall(PixelFnCall),
}

#[derive(Debug, Clone)]
pub enum MatchComparisonValue {
    Scalar(ScalarExprNode),
    Pixel(PixelExprType),
}

#[derive(Debug, Clone)]
pub struct MatchComparatorNode {
    pub op_type: MatchOpType,
    pub cmp_val: MatchComparisonValue,
}

#[derive(Debug, Clone)]
pub enum MatchReturnValue {
    Pixel(PixelExprType),
    Operator(OperatorNode),
}

#[derive(Debug, Clone)]
pub struct MatchExprOpNode {
    pub match_value: MatchComparisonValue,
    pub match_comparator_node: Option<MatchComparatorNode>,
    pub match_return_value_node: Box<MatchReturnValue>,
}

#[derive(Debug, Clone)]
pub enum OperatorNode {
    UnaryNegationOp(),
    MatchExprOp(MatchExprOpNode),
}

#[derive(Debug, Clone)]
pub struct BinaryScalarOpNode {
    pub lhs: ScalarExprNode,
    pub op: BinaryOpType,
    pub rhs: ScalarExprNode,
}

#[derive(Debug, Clone)]
pub enum ScalarFnOp {
    Min(),
    Max(),
    Square(),
    Sqrt(),
}

#[derive(Debug, Clone)]
pub enum PixelFnOp {
    Center(),
    Neighbors(i64, i64),
    ColorScale(f64),
    ColorAdd(),
    ColorNorm(),
}

#[derive(Debug, Clone)]
pub struct PixelFnCall {
    pub op: PixelFnOp,
    pub args: Vec<PixelExprType>,
}

#[derive(Debug, Clone)]
pub struct ScalarFnCall {
    pub op: ScalarFnOp,
    pub args: Vec<ScalarExprNode>,
}

#[derive(Debug, Clone)]
pub enum ScalarExprNode {
    ScalarFn(ScalarFnCall),
    SubExpr(Box<ScalarExprNode>),
    Scalar(ScalarNode),
    BinaryOp(Box<BinaryScalarOpNode>),
}

#[derive(Debug, Clone)]
pub struct SliceRangeNode {
    pub lower_bound: Option<ScalarExprNode>,
    pub upper_bound: Option<ScalarExprNode>,
}

#[derive(Debug, Clone)]
pub struct SelectorCtxNode {
    pub y_slice_range: Option<Box<SliceRangeNode>>,
    pub x_slice_range: Option<Box<SliceRangeNode>>,
}

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub selector_ctx: Option<SelectorCtxNode>,
    pub op_nodes: Vec<OperatorNode>,
}

#[derive(Debug, Clone)]
pub struct IqAstRootNode {
    pub exprs: Vec<ExprNode>,
}
