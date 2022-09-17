use crate::context::*;

pub fn access_scalar_annotated_ctx_attr(
    ctx: &AnnotatedPixelContext,
    attr: &String,
) -> AnnotatedFloatContext {
    if attr.eq_ignore_ascii_case("y") {
        return AnnotatedFloatContext::from_iter_with_annotation(
            ctx.iter_annotations(),
            |(pixel, annot)| (pixel.clone(), annot.y as f64),
        );
    } else if attr.eq_ignore_ascii_case("x") {
        return AnnotatedFloatContext::from_iter_with_annotation(
            ctx.iter_annotations(),
            |(pixel, annot)| (pixel.clone(), annot.x as f64),
        );
    }
    for (i, x) in ["r", "g", "b", "a"].iter().enumerate() {
        if attr.eq_ignore_ascii_case(x) {
            return AnnotatedFloatContext::from_iter_with_annotation(
                ctx.iter_annotations(),
                |(pixel, annot)| (pixel.clone(), annot.c[i] as f64),
            );
        }
    }
    panic!("Unknown attribute: {:?}", attr)
}

pub fn access_scalar_attr<T>(ctx: &Context<T>, attr: &String) -> f64 {
    if attr.eq_ignore_ascii_case("h") {
        ctx.height() as f64
    } else if attr.eq_ignore_ascii_case("w") {
        ctx.width() as f64
    } else {
        panic!("Unknown attribute: {:?}", attr)
    }
}
