use crate::context::*;

pub fn center(ctx: &BasicContext) -> AnnotatedPixelContext {
    AnnotatedPixelContext::like(ctx, &ctx.center())
}

pub fn neighbors(
    ctx: &BasicContext,
    arg: &AnnotatedPixelContext,
    dy: u64,
    dx: u64,
) -> AnnotatedPixelContext {
    AnnotatedPixelContext::from_iter_with_annotation(ctx.iter(), |pixel| {
        let ny = pixel.y + dy as u32;
        let nx = pixel.x + dx as u32;
        let default = IqPixel {
            y: ny,
            x: nx,
            c: [0, 0, 0, 0],
        };
        (
            pixel.clone(),
            IqPixel {
                y: ny,
                x: nx,
                c: arg.get_annotation_at_loc((ny, nx)).unwrap_or(&default).c,
            },
        )
    })
}
