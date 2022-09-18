use crate::ast::*;
use crate::attrs;
use crate::context::{AnnotatedFloatContext, AnnotatedPixelContext, BasicContext, IqPixel};
use crate::ctx_ops;
use crate::float_ops;

pub trait Evalulate<T> {
    fn eval(&self, image_ctx: &BasicContext) -> T;
}

impl Evalulate<BasicContext> for IqAstRootNode {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        BasicContext::from_contexts(
            (*self.exprs)
                .iter()
                .map(|expr| expr.eval(image_ctx))
                .collect(),
        )
    }
}

impl Evalulate<BasicContext> for ExprNode {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        let mut selected_ctx = match &self.selector_ctx {
            None => image_ctx.clone(),
            Some(selector_ctx) => selector_ctx.eval(image_ctx),
        };

        for op in &self.op_nodes {
            selected_ctx = op.eval(&selected_ctx);
        }

        selected_ctx
    }
}

impl SliceRangeNode {
    fn default_x(image_ctx: &BasicContext) -> Self {
        Self::with_bounds(0, image_ctx.width().into())
    }

    fn default_y(image_ctx: &BasicContext) -> Self {
        Self::with_bounds(0, image_ctx.height().into())
    }

    fn with_bounds(lower: i64, upper: i64) -> Self {
        SliceRangeNode {
            lower_bound: Some(ScalarExprNode::Scalar(ScalarNode::Integer(lower))),
            upper_bound: Some(ScalarExprNode::Scalar(ScalarNode::Integer(upper))),
        }
    }
}

impl Evalulate<BasicContext> for SelectorCtxNode {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        let y_slice_range = match &self.y_slice_range {
            None => Box::new(SliceRangeNode::default_y(image_ctx)),
            Some(y_slice_range) => y_slice_range.clone(),
        };

        let x_slice_range = match &self.x_slice_range {
            None => Box::new(SliceRangeNode::default_x(image_ctx)),
            Some(x_slice_range) => x_slice_range.clone(),
        };

        let y_bounds = y_slice_range.eval(image_ctx);
        let x_bounds = x_slice_range.eval(image_ctx);

        image_ctx.subcontext(y_bounds, x_bounds)
    }
}

impl Evalulate<(Option<u32>, Option<u32>)> for SliceRangeNode {
    fn eval(&self, image_ctx: &BasicContext) -> (Option<u32>, Option<u32>) {
        let lower_bound = match &self.lower_bound {
            None => None,
            Some(lower_bound) => {
                let floating_lower_bound: f64 = *lower_bound.eval(image_ctx).first();
                Some(floating_lower_bound.round() as u32)
            }
        };
        let upper_bound = match &self.upper_bound {
            None => None,
            Some(upper_bound) => {
                let floating_upper_bound: f64 = *upper_bound.eval(image_ctx).first();
                Some(floating_upper_bound.round() as u32)
            }
        };

        (lower_bound, upper_bound)
    }
}

impl Evalulate<AnnotatedFloatContext> for ScalarExprNode {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedFloatContext {
        match &self {
            Self::ScalarFn(fncall_node) => fncall_node.eval(image_ctx),
            Self::SubExpr(subexpr_node) => subexpr_node.eval(image_ctx),
            Self::Scalar(scalar_node) => scalar_node.eval(image_ctx),
            Self::BinaryOp(binary_op_node) => binary_op_node.eval(image_ctx),
        }
    }
}

impl Evalulate<AnnotatedFloatContext> for ScalarFnCall {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedFloatContext {
        let mut evaluated_args = (*self.args).iter().map(|arg| arg.eval(image_ctx));

        match &self.op {
            ScalarFnOp::Min() => float_ops::min(&evaluated_args.collect()),
            ScalarFnOp::Max() => float_ops::max(&evaluated_args.collect()),
            ScalarFnOp::Square() => float_ops::square(&evaluated_args.next().unwrap()),
            ScalarFnOp::Sqrt() => float_ops::sqrt(&evaluated_args.next().unwrap()),
        }
    }
}

impl Evalulate<AnnotatedFloatContext> for ScalarNode {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedFloatContext {
        match &self {
            ScalarNode::Float(n) => AnnotatedFloatContext::like(image_ctx, n),
            ScalarNode::Integer(n) => AnnotatedFloatContext::like(image_ctx, &(*n as f64)),
            ScalarNode::SelectorScalar(selector_scalar_node) => {
                AnnotatedFloatContext::like(image_ctx, &selector_scalar_node.eval(image_ctx))
            }
            ScalarNode::PixelScalar(pixel_expr, attr_access) => {
                attrs::access_scalar_annotated_ctx_attr(
                    &pixel_expr.eval(image_ctx),
                    &attr_access.key,
                )
            }
        }
    }
}

impl Evalulate<f64> for SelectorScalarNode {
    fn eval(&self, image_ctx: &BasicContext) -> f64 {
        attrs::access_scalar_attr(&self.selector_ctx.eval(image_ctx), &self.accessed_attr.key)
    }
}

impl Evalulate<AnnotatedFloatContext> for BinaryScalarOpNode {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedFloatContext {
        let lhs = self.lhs.eval(image_ctx);
        let rhs = self.rhs.eval(image_ctx);
        match &self.op {
            BinaryOpType::Add() => float_ops::add(&lhs, &rhs),
            BinaryOpType::Sub() => float_ops::sub(&lhs, &rhs),
            BinaryOpType::Div() => float_ops::div(&lhs, &rhs),
            BinaryOpType::Mul() => float_ops::mul(&lhs, &rhs),
        }
    }
}

impl Evalulate<BasicContext> for OperatorNode {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        match &self {
            Self::UnaryNegationOp() => float_ops::negate(image_ctx),
            Self::MatchExprOp(op) => op.eval(image_ctx),
        }
    }
}

fn match_compare<T: PartialEq + PartialOrd>(op_type: &MatchOpType, lhs: T, rhs: T) -> bool {
    match op_type {
        MatchOpType::Lt() => lhs < rhs,
        MatchOpType::Lte() => lhs <= rhs,
        MatchOpType::Gt() => lhs > rhs,
        MatchOpType::Gte() => lhs >= rhs,
        MatchOpType::Eq() => lhs == rhs,
        MatchOpType::Neq() => lhs != rhs,
    }
}

impl Evalulate<BasicContext> for MatchExprOpNode {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        let match_comp_lhs = &self.match_value;
        let (matched_ctx, else_context) = match &self.match_comparator_node {
            None => (image_ctx.clone(), BasicContext::empty()),
            Some(match_comparator) => match (match_comp_lhs, &match_comparator.cmp_val) {
                (
                    MatchComparisonValue::Scalar(lhs_scalar_expr),
                    MatchComparisonValue::Scalar(rhs_scalar_expr),
                ) => {
                    let lhs_terms: AnnotatedFloatContext = lhs_scalar_expr.eval(image_ctx);
                    let rhs_terms: AnnotatedFloatContext = rhs_scalar_expr.eval(image_ctx);
                    let mut matched_ctx = BasicContext::empty();
                    let mut else_context = BasicContext::empty();

                    for (point, annotation) in lhs_terms.iter_annotations() {
                        if match_compare(
                            &match_comparator.op_type,
                            annotation,
                            rhs_terms.get_annotation(point).unwrap(),
                        ) {
                            matched_ctx.insert(point.clone());
                        } else {
                            else_context.insert(point.clone());
                        }
                    }

                    (matched_ctx, else_context)
                }
                (
                    MatchComparisonValue::Pixel(lhs_pixel_expr),
                    MatchComparisonValue::Pixel(rhs_pixel_expr),
                ) => {
                    let lhs_terms: AnnotatedPixelContext = lhs_pixel_expr.eval(image_ctx);
                    let rhs_terms: AnnotatedPixelContext = rhs_pixel_expr.eval(image_ctx);
                    let mut matched_ctx = BasicContext::empty();
                    let mut else_context = BasicContext::empty();

                    for (point, annotation) in lhs_terms.iter_annotations() {
                        if match_compare(
                            &match_comparator.op_type,
                            annotation,
                            rhs_terms.get_annotation(point).unwrap(),
                        ) {
                            matched_ctx.insert(annotation.clone());
                        } else {
                            else_context.insert(annotation.clone());
                        }
                    }

                    (matched_ctx, else_context)
                }
                _ => panic!("match terms have incompatible types"),
            },
        };
        let matched_outputs = self.match_return_value_node.eval(&matched_ctx);

        if let Some(else_block) = &self.else_return_value_node {
            BasicContext::from_contexts(vec![matched_outputs, else_block.eval(&else_context)])
        } else {
            matched_outputs
        }
    }
}

impl Evalulate<BasicContext> for MatchReturnValue {
    fn eval(&self, image_ctx: &BasicContext) -> BasicContext {
        match self {
            MatchReturnValue::Pixel(pixel_expr) => BasicContext::from_iter(
                pixel_expr
                    .eval(image_ctx)
                    .iter_annotations()
                    .map(|(_, annotation)| annotation)
                    .cloned(),
                |annotation| annotation,
            ),
            MatchReturnValue::Operator(operator) => operator.eval(image_ctx),
        }
    }
}

impl Evalulate<AnnotatedPixelContext> for PixelExprType {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedPixelContext {
        match self {
            PixelExprType::Explicit(pixelexpr) => pixelexpr.eval(image_ctx),
            PixelExprType::CurrentPixel() => {
                AnnotatedPixelContext::from_iter_with_annotation(image_ctx.iter(), |point| {
                    (point.clone(), point.clone())
                })
            }
            PixelExprType::FnCall(pixel_fn_call) => pixel_fn_call.eval(image_ctx),
        }
    }
}

impl Evalulate<AnnotatedPixelContext> for PixelFnCall {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedPixelContext {
        let mut evaluated_args = (*self.args).iter().map(|arg| arg.eval(image_ctx));
        match self.op {
            PixelFnOp::Center() => ctx_ops::center(image_ctx),
            PixelFnOp::Neighbors(dy, dx) => {
                ctx_ops::neighbors(&evaluated_args.next().unwrap(), dy, dx)
            }
            PixelFnOp::ColorScale(scale_factor) => {
                ctx_ops::color_scale(&evaluated_args.next().unwrap(), scale_factor)
            }
            PixelFnOp::ColorAdd() => ctx_ops::color_add(&evaluated_args.collect()),
            PixelFnOp::ColorNorm() => ctx_ops::color_norm(&evaluated_args.next().unwrap()),
            PixelFnOp::AlphaBlend(blend) => {
                ctx_ops::alpha_blend(&evaluated_args.next().unwrap(), blend)
            }
        }
    }
}

impl Evalulate<AnnotatedPixelContext> for PixelNode {
    fn eval(&self, image_ctx: &BasicContext) -> AnnotatedPixelContext {
        let x_values = self.x_expr.eval(image_ctx);
        let y_values = self.y_expr.eval(image_ctx);
        let r_values = self.r_expr.eval(image_ctx);
        let g_values = self.g_expr.eval(image_ctx);
        let b_values = self.b_expr.eval(image_ctx);
        let a_values = self.a_expr.eval(image_ctx);

        let mut annotated_ctx = AnnotatedPixelContext::empty();
        for pixel in image_ctx.iter() {
            annotated_ctx.insert_with_annotation(
                pixel.clone(),
                IqPixel {
                    x: x_values.get_annotation(pixel).unwrap().round() as u32,
                    y: y_values.get_annotation(pixel).unwrap().round() as u32,
                    c: [
                        r_values.get_annotation(pixel).unwrap().round() as i64,
                        g_values.get_annotation(pixel).unwrap().round() as i64,
                        b_values.get_annotation(pixel).unwrap().round() as i64,
                        a_values.get_annotation(pixel).unwrap().round() as i64,
                    ],
                },
            );
        }

        annotated_ctx
    }
}
