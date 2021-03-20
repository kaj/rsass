//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1960.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo:not(.missing) {\
            \n  a: b;\
            \n\
            \n  &:hover { c: d; }\
            \n}\
            \n\
            \nbar {\
            \n  @extend .missing;\
            \n}"
        )
        .unwrap(),
        "foo:not(.missing):not(bar) {\
        \n  a: b;\
        \n}\
        \nfoo:not(.missing):not(bar):hover {\
        \n  c: d;\
        \n}\
        \n"
    );
}
