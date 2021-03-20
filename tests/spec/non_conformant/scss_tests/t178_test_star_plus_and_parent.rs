//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/178_test_star_plus_and_parent.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {*+html & {a: b}}\
            \n"
        )
        .unwrap(),
        "* + html foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
