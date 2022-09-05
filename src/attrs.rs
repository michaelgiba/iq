use crate::context::*;

pub fn access_pixel_ctx_attr<T>(ctx: &Context<T>, attr: &String) -> AnnotatedPixelContext {
    if attr.eq_ignore_ascii_case("c") {
        let midpoint = ctx.midpoint();

        AnnotatedPixelContext::like(
            ctx,
            &IqPixel {
                x: midpoint.0,
                y: midpoint.1,
                c: [255, 255, 255, 255],
            },
        )
    } else {
        panic!("Unknown attribute: {:?}", attr)
    }
}

pub fn access_scalar_annotated_ctx_attr(
    ctx: &AnnotatedPixelContext,
    attr: &String,
) -> AnnotatedFloatContext {
    if attr.eq_ignore_ascii_case("y") {
        let mut annotated_ctx = AnnotatedFloatContext::empty();
        for (pixel, annot) in ctx.iter_annotations() {
            annotated_ctx.insert_with_annotation(pixel.clone(), annot.y as f64)
        }
        return annotated_ctx;
    } else if attr.eq_ignore_ascii_case("x") {
        let mut annotated_ctx = AnnotatedFloatContext::empty();
        for (pixel, annot) in ctx.iter_annotations() {
            annotated_ctx.insert_with_annotation(pixel.clone(), annot.x as f64)
        }
        return annotated_ctx;
    }
    for (i, x) in ["r", "g", "b", "a"].iter().enumerate() {
        if attr.eq_ignore_ascii_case(x) {
            let mut annotated_ctx = AnnotatedFloatContext::empty();
            for (pixel, annot) in ctx.iter_annotations() {
                annotated_ctx.insert_with_annotation(pixel.clone(), annot.c[i] as f64)
            }
            return annotated_ctx;
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
