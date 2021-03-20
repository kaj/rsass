//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/parent_selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  &:hover {a: b}\
            \n  bar &.baz {c: d}}\
            \n"
        )
        .unwrap(),
        "foo:hover {\
        \n  a: b;\
        \n}\
        \nbar foo.baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}
