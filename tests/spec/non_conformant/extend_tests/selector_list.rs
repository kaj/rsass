//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/selector_list.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {x: y}\
            \n\
            \n// Extending a selector list is equivalent to writing two @extends.\
            \n.baz {@extend .foo, .bar}\
            \n\
            \n// The selector list should be parsed after interpolation is resolved.\
            \n.bang {@extend .foo #{\",\"} .bar}\
            \n"
        )
        .unwrap(),
        ".foo, .bang, .baz {\
        \n  a: b;\
        \n}\
        \n.bar, .bang, .baz {\
        \n  x: y;\
        \n}\
        \n"
    );
}
