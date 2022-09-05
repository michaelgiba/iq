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
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, a_annot) in a.iter_annotations() {
        let b_annot = b.get_annotation(pixel).unwrap();
        let max = if a_annot < b_annot {
            *a_annot
        } else {
            *b_annot
        };
        out.insert_with_annotation(pixel.clone(), max);
    }
    out
}

pub fn min(args: &Vec<AnnotatedFloatContext>) -> AnnotatedFloatContext {
    if args.len() == 0 {
        AnnotatedFloatContext::empty()
    } else {
        args.into_iter()
            .cloned()
            .reduce(|accum, item| min2(&accum, &item))
            .unwrap()
            .clone()
    }
}

fn max2(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, a_annot) in a.iter_annotations() {
        let b_annot = b.get_annotation(pixel).unwrap();
        let max = if a_annot > b_annot {
            *a_annot
        } else {
            *b_annot
        };
        out.insert_with_annotation(pixel.clone(), max);
    }
    out
}

pub fn max(args: &Vec<AnnotatedFloatContext>) -> AnnotatedFloatContext {
    if args.len() == 0 {
        AnnotatedFloatContext::empty()
    } else {
        args.into_iter()
            .cloned()
            .reduce(|accum, item| max2(&accum, &item))
            .unwrap()
            .clone()
    }
}

pub fn square(arg: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, annot) in arg.iter_annotations() {
        out.insert_with_annotation(pixel.clone(), (*annot).powi(2));
    }
    out
}

pub fn sqrt(arg: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, annot) in arg.iter_annotations() {
        out.insert_with_annotation(pixel.clone(), (*annot).sqrt());
    }
    out
}

pub fn add(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, a_annot) in a.iter_annotations() {
        let b_annot = b.get_annotation(pixel).unwrap();
        out.insert_with_annotation(pixel.clone(), a_annot + b_annot);
    }
    out
}

pub fn sub(l: &AnnotatedFloatContext, r: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(l, r);
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, l_annot) in l.iter_annotations() {
        let r_annot = r.get_annotation(pixel).unwrap();
        // println!("{} {}", l_annot, r_annot);
        out.insert_with_annotation(pixel.clone(), l_annot - r_annot);
    }
    out
}

pub fn div(l: &AnnotatedFloatContext, r: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(l, r);
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, l_annot) in l.iter_annotations() {
        let r_annot = r.get_annotation(pixel).unwrap();
        out.insert_with_annotation(pixel.clone(), l_annot / r_annot);
    }
    out
}

pub fn mul(a: &AnnotatedFloatContext, b: &AnnotatedFloatContext) -> AnnotatedFloatContext {
    assert_compatible_contexts(a, b);
    let mut out = AnnotatedFloatContext::empty();
    for (pixel, a_annot) in a.iter_annotations() {
        let b_annot = b.get_annotation(pixel).unwrap();
        out.insert_with_annotation(pixel.clone(), a_annot * b_annot);
    }
    out
}

pub fn negate(arg: &BasicContext) -> BasicContext {
    let mut out = BasicContext::empty();
    for pixel in arg.iter() {
        out.insert(pixel.negate());
    }
    out
}
