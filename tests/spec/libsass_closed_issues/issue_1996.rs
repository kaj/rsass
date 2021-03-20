//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1996.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: \"bar\";\
            \n\
            \n%class //#{$foo}\
            \n{\
            \n  &_baz {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ""
    );
}
