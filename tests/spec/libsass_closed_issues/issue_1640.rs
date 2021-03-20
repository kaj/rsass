//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1640.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo() {\
            \n    @if false {\
            \n      a { b: c }\
            \n    } @else {\
            \n      @content;\
            \n    }\
            \n}\
            \n\
            \n@include foo() {\
            \n  .foo {\
            \n    bar: baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
