use crate::context::*;
use std::vec::Vec;

pub fn center(ctx: &BasicContext) -> AnnotatedPixelContext {
    AnnotatedPixelContext::like(ctx, &ctx.center())
}

pub fn neighbors(arg: &AnnotatedPixelContext, dy: i64, dx: i64) -> AnnotatedPixelContext {
    AnnotatedPixelContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        let ny = (annot.y as i64 + dy) as u32;
        let nx = (annot.x as i64 + dx) as u32;
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

pub fn color_scale(arg: &AnnotatedPixelContext, scale_factor: f64) -> AnnotatedPixelContext {
    AnnotatedPixelContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        (
            pixel.clone(),
            IqPixel {
                y: pixel.y,
                x: pixel.x,
                c: [
                    (annot.c[0] as f64 * scale_factor) as i64,
                    (annot.c[1] as f64 * scale_factor) as i64,
                    (annot.c[2] as f64 * scale_factor) as i64,
                    annot.c[3],
                ],
            },
        )
    })
}

pub fn color_add(args: &Vec<AnnotatedPixelContext>) -> AnnotatedPixelContext {
    if args.is_empty() {
        return AnnotatedPixelContext::empty();
    }

    let red_channels: Vec<AnnotatedFloatContext> = args
        .iter()
        .map(|pixel_context| {
            AnnotatedFloatContext::from_iter_with_annotation(
                pixel_context.iter_annotations(),
                |(pixel, annot)| (pixel.clone(), annot.c[0] as f64),
            )
        })
        .collect();
    let green_channels: Vec<AnnotatedFloatContext> = args
        .iter()
        .map(|pixel_context| {
            AnnotatedFloatContext::from_iter_with_annotation(
                pixel_context.iter_annotations(),
                |(pixel, annot)| (pixel.clone(), annot.c[0] as f64),
            )
        })
        .collect();
    let blue_channels: Vec<AnnotatedFloatContext> = args
        .iter()
        .map(|pixel_context| {
            AnnotatedFloatContext::from_iter_with_annotation(
                pixel_context.iter_annotations(),
                |(pixel, annot)| (pixel.clone(), annot.c[0] as f64),
            )
        })
        .collect();

    let mut merged_red_channel: AnnotatedFloatContext = AnnotatedFloatContext::empty();
    let mut merged_green_channel: AnnotatedFloatContext = AnnotatedFloatContext::empty();
    let mut merged_blue_channel: AnnotatedFloatContext = AnnotatedFloatContext::empty();

    red_channels.iter().for_each(|r_channel_ctx| {
        for (pixel, &value) in r_channel_ctx.iter_annotations() {
            let loc = (pixel.y, pixel.x);
            if let Some(annot) = merged_red_channel.get_annotation_at_loc(loc) {
                merged_red_channel.update_annot_at_loc(loc, annot + value);
            } else {
                merged_red_channel.insert_with_annotation(pixel.clone(), value);
            }
        }
    });
    green_channels.iter().for_each(|g_channel_ctx| {
        for (pixel, &value) in g_channel_ctx.iter_annotations() {
            let loc = (pixel.y, pixel.x);
            if let Some(annot) = merged_green_channel.get_annotation_at_loc(loc) {
                merged_green_channel.update_annot_at_loc(loc, annot + value);
            } else {
                merged_green_channel.insert_with_annotation(pixel.clone(), value);
            }
        }
    });
    blue_channels.iter().for_each(|b_channel_ctx| {
        for (pixel, &value) in b_channel_ctx.iter_annotations() {
            let loc = (pixel.y, pixel.x);
            if let Some(annot) = merged_blue_channel.get_annotation_at_loc(loc) {
                merged_blue_channel.update_annot_at_loc(loc, annot + value);
            } else {
                merged_blue_channel.insert_with_annotation(pixel.clone(), value);
            }
        }
    });

    AnnotatedPixelContext::from_iter_with_annotation(args.first().unwrap().iter(), |pixel| {
        let r = *merged_red_channel
            .get_annotation_at_loc((pixel.y, pixel.x))
            .unwrap() as i64;
        let g = *merged_green_channel
            .get_annotation_at_loc((pixel.y, pixel.x))
            .unwrap() as i64;
        let b = *merged_blue_channel
            .get_annotation_at_loc((pixel.y, pixel.x))
            .unwrap() as i64;

        (
            pixel.clone(),
            IqPixel {
                y: pixel.y,
                x: pixel.x,
                c: [r, g, b, pixel.c[3]],
            },
        )
    })
}

pub fn color_norm(arg: &AnnotatedPixelContext) -> AnnotatedPixelContext {
    if arg.count() == 0 {
        return AnnotatedPixelContext::empty();
    }

    let r_bounds = arg
        .iter_annotations()
        .map(|(_, annot)| (annot.c[0], annot.c[0]))
        .reduce(|accum, rval| {
            (
                std::cmp::min(accum.0, rval.0),
                std::cmp::max(accum.1, rval.1),
            )
        })
        .unwrap();
    let g_bounds = arg
        .iter_annotations()
        .map(|(_, annot)| (annot.c[1], annot.c[1]))
        .reduce(|accum, rval| {
            (
                std::cmp::min(accum.0, rval.0),
                std::cmp::max(accum.1, rval.1),
            )
        })
        .unwrap();
    let b_bounds = arg
        .iter_annotations()
        .map(|(_, annot)| (annot.c[2], annot.c[2]))
        .reduce(|accum, rval| {
            (
                std::cmp::min(accum.0, rval.0),
                std::cmp::max(accum.1, rval.1),
            )
        })
        .unwrap();

    let r_range = if (r_bounds.1 - r_bounds.0) == 0 {
        1.0
    } else {
        (r_bounds.1 - r_bounds.0) as f64
    };

    let g_range = if (g_bounds.1 - g_bounds.0) == 0 {
        1.0
    } else {
        (g_bounds.1 - g_bounds.0) as f64
    };

    let b_range = if (b_bounds.1 - b_bounds.0) == 0 {
        1.0
    } else {
        (b_bounds.1 - b_bounds.0) as f64
    };

    let r_inv = 1.0 / r_range;
    let g_inv = 1.0 / g_range;
    let b_inv = 1.0 / b_range;

    AnnotatedPixelContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        (
            pixel.clone(),
            IqPixel {
                y: pixel.y,
                x: pixel.x,
                c: [
                    (((annot.c[0] - r_bounds.0) as f64 * r_inv) * 255.0) as i64,
                    (((annot.c[1] - g_bounds.0) as f64 * g_inv) * 255.0) as i64,
                    (((annot.c[2] - b_bounds.0) as f64 * b_inv) * 255.0) as i64,
                    annot.c[3],
                ],
            },
        )
    })
}

pub fn alpha_blend(arg: &AnnotatedPixelContext, blend: f64) -> AnnotatedPixelContext {
    AnnotatedPixelContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        (
            pixel.clone(),
            IqPixel {
                y: pixel.y,
                x: pixel.x,
                c: [
                    annot.c[0],
                    annot.c[1],
                    annot.c[2],
                    (annot.c[3] as f64 * blend) as i64,
                ],
            },
        )
    })
}
