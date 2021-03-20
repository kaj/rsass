//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2053.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo[disabled] {\
            \n    @extend .foo;\
            \n    background: blue;\
            \n  }\
            \n.bar[disabled] {\
            \n  foo {\
            \n    @extend .bar;\
            \n    background: blue;\
            \n  }\
            \n}\
            \nfoo {\
            \n  .baz[disabled] {\
            \n    @extend .baz;\
            \n    background: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo[disabled] {\
        \n  background: blue;\
        \n}\
        \n.bar[disabled] foo {\
        \n  background: blue;\
        \n}\
        \nfoo .baz[disabled] {\
        \n  background: blue;\
        \n}\
        \n"
    );
}
