//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1526.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: (1--em-2--em);\
            \n  baz: (1--em - 2--em);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 1 --em-2--em;\
        \n  baz: 1 --em-2 --em;\
        \n}\
        \n"
    );
}
