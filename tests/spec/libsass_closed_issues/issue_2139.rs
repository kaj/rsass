//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2139.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  color: #FFF;\
            \n}\
            \n\
            \n.bar .baz {\
            \n  @extend .foo;\
            \n\
            \n  border: \'1px\';\
            \n}\
            \n\
            \n*:not(.foo) {\
            \n  display: none;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo, .bar .baz {\
        \n  color: #FFF;\
        \n}\
        \n.bar .baz {\
        \n  border: \"1px\";\
        \n}\
        \n*:not(.foo) {\
        \n  display: none;\
        \n}\
        \n"
    );
}
