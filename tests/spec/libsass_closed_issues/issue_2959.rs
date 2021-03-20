//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2959.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%color {\
            \n\tcolor: blue;\
            \n}\
            \n\
            \n@mixin getOverridedSelector {\
            \n\t&#{&} {\
            \n\t\t@content;\
            \n\t}\
            \n}\
            \n\
            \n.foo {\
            \n\t@include getOverridedSelector {\
            \n\t\t@extend %color;\
            \n\t}\
            \n}\
            \n\
            \n.bar {\
            \n\t@include getOverridedSelector {\
            \n\t\tcolor: red;\
            \n\t}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo.foo {\
        \n  color: blue;\
        \n}\
        \n.bar.bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
