use iq::context::BasicContext;

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
}
