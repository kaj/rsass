//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2031/extended-not.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ":not(.foo) {\r\
            \n  content: test;\r\
            \n}\r\
            \n.bar, .baz {\r\
            \n  @extend .foo;\r\
            \n}\r\
            \n\r\
            \ntest {\r\
            \n  content: selector-extend(\":not(.foo)\", \".foo\", \".bar\");\r\
            \n}"
        )
        .unwrap(),
        ":not(.foo):not(.bar):not(.baz) {\
        \n  content: test;\
        \n}\
        \ntest {\
        \n  content: :not(.foo):not(.bar);\
        \n}\
        \n"
    );
}
