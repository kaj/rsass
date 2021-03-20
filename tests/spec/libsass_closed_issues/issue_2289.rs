//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2289.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo:baz:baz {\
            \n  float: left;\
            \n}\
            \n\
            \n.bar {\
            \n  @extend .foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo:baz:baz, .bar:baz:baz {\
        \n  float: left;\
        \n}\
        \n"
    );
}
