use crate::context::*;

fn are_compatible_contexts<T>(a: &Context<T>, b: &Context<T>) -> bool {
    a.x_bounds() == b.x_bounds() && a.y_bounds() == b.y_bounds() && a.count() == b.count()
}

fn assert_compatible_contexts<T>(a: &Context<T>, b: &Context<T>) {
    if !are_compatible_contexts(a, b) {
        panic!(
            "Incompatible contexts: a= {:} b = {:}",
            a.describe(),
            b.describe()
        )
    }
}

fn min2(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    AnnotatedFloatContext::from_iter_with_annotation(a.iter_annotations(), |(pixel, a_annot)| {
        let b_annot = b.get_annotation(pixel).unwrap();
        (
            pixel.clone(),
            if a_annot < b_annot {
                *a_annot
            } else {
                *b_annot
            },
        )
    })
}

pub fn min(args: &Vec<AnnotatedFloatContext>) -> AnnotatedFloatContext {
    if args.is_empty() {
        AnnotatedFloatContext::empty()
    } else {
        args.iter()
            .cloned()
            .reduce(|accum, item| min2(&accum, &item))
            .unwrap()
    }
}

fn max2(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);

    AnnotatedFloatContext::from_iter_with_annotation(a.iter_annotations(), |(pixel, a_annot)| {
        let b_annot = b.get_annotation(pixel).unwrap();
        (
            pixel.clone(),
            if a_annot > b_annot {
                *a_annot
            } else {
                *b_annot
            },
        )
    })
}

pub fn max(args: &Vec<AnnotatedFloatContext>) -> AnnotatedFloatContext {
    if args.is_empty() {
        AnnotatedFloatContext::empty()
    } else {
        args.iter()
            .cloned()
            .reduce(|accum, item| max2(&accum, &item))
            .unwrap()
    }
}

pub fn square(arg: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    AnnotatedFloatContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        (pixel.clone(), annot.powi(2))
    })
}

pub fn sqrt(arg: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    AnnotatedFloatContext::from_iter_with_annotation(arg.iter_annotations(), |(pixel, annot)| {
        (pixel.clone(), annot.sqrt())
    })
}

pub fn add(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);
    AnnotatedFloatContext::from_iter_with_annotation(a.iter_annotations(), |(pixel, a_annot)| {
        let b_annot = b.get_annotation(pixel).unwrap();
        (pixel.clone(), a_annot + b_annot)
    })
}

pub fn sub(l: &AnnotatedFloatContext, r: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(l, r);
    AnnotatedFloatContext::from_iter_with_annotation(l.iter_annotations(), |(pixel, l_annot)| {
        let r_annot = r.get_annotation(pixel).unwrap();
        (pixel.clone(), l_annot - r_annot)
    })
}

pub fn div(l: &AnnotatedFloatContext, r: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(l, r);
    AnnotatedFloatContext::from_iter_with_annotation(l.iter_annotations(), |(pixel, l_annot)| {
        let r_annot = r.get_annotation(pixel).unwrap();
        (pixel.clone(), l_annot / r_annot)
    })
}

pub fn mul(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);
    AnnotatedFloatContext::from_iter_with_annotation(a.iter_annotations(), |(pixel, a_annot)| {
        let b_annot = b.get_annotation(pixel).unwrap();
        (pixel.clone(), a_annot * b_annot)
    })
}

pub fn negate(arg: &BasicContext) -> BasicContext {
    BasicContext::from_iter(arg.iter(), |pixel| pixel.negate())
}
