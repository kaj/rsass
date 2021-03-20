//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/046_test_parent_selector_with_subject.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar &.baz! .bip {a: b}}\
            \n\
            \nfoo bar {\
            \n  bar &.baz! .bip {c: d}}\
            \n"
        )
        .unwrap(),
        "bar foo.baz! .bip {\
        \n  a: b;\
        \n}\
        \nbar foo bar.baz! .bip {\
        \n  c: d;\
        \n}\
        \n"
    );
}
