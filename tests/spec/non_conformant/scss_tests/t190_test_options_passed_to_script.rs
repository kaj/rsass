//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/190_test_options_passed_to_script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {color: darken(black, 10%)}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: black;\
        \n}\
        \n"
    );
}
