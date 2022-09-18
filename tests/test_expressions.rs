use iq::context::BasicContext;
use std::fs;
use std::path::{Path, PathBuf};

fn test_file_path(rel_path: &str) -> PathBuf {
    let root = env!("CARGO_MANIFEST_DIR");
    Path::new(root).join("tests/test_files").join(rel_path)
}

fn test_file_contents(rel_path: &str) -> String {
    fs::read_to_string(test_file_path(rel_path)).unwrap()
}

#[test]
fn handles_empty_input() {
    assert_eq!(
        BasicContext::empty(),
        iq::execute(BasicContext::empty(), String::from(""))
    );
}

#[test]
fn handles_identity() {
    assert_eq!(
        BasicContext::empty(),
        iq::execute(BasicContext::empty(), String::from("_ => _"))
    );
    assert_eq!(
        BasicContext::blank(10, 10),
        iq::execute(BasicContext::blank(10, 10), String::from("_ => _"))
    );
    assert_eq!(
        BasicContext::blank(10, 10),
        iq::execute(
            BasicContext::blank(10, 10),
            String::from("_ => p(_.y, _.x, _.r, _.g, _.b)")
        )
    );
    assert_eq!(
        BasicContext::blank(10, 10),
        iq::execute(
            BasicContext::blank(10, 10),
            test_file_contents("scripts/identity.iq")
        )
    );
}

#[test]
fn handles_context_ops() {
    assert_eq!(
        BasicContext::blank_with_default(10, 10, [0, 0, 0, 255]),
        iq::execute(
            BasicContext::blank_with_default(10, 10, [255, 255, 255, 255]),
            test_file_contents("scripts/color_scale.iq")
        )
    );

    // Just to check sobel doesn't crash.
    let output_ctx = iq::execute(
        BasicContext::blank_with_default(10, 10, [255, 255, 255, 255]),
        test_file_contents("scripts/sobel_edge_detection.iq"),
    );

    assert_eq!(output_ctx.clone(), output_ctx);
}
