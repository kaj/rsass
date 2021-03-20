//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_942.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$v: \".foo \\\
            \n.bar\";\
            \n\
            \n#{$v} {\
            \n\tcolor: #F00;\
            \n}\
            \n\
            \ndiv {\
            \n\tcontent: \"foo\\\
            \nbar\";\
            \n}"
        )
        .unwrap(),
        ".foo .bar {\
        \n  color: #F00;\
        \n}\
        \ndiv {\
        \n  content: \"foobar\";\
        \n}\
        \n"
    );
}
