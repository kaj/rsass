//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/180_test_interpolation_with_bracket_on_next_line.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a.#{\"foo\"} b\
            \n{color: red}\
            \n"
        )
        .unwrap(),
        "a.foo b {\
        \n  color: red;\
        \n}\
        \n"
    );
}
