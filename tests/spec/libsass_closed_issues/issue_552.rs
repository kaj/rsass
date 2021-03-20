//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_552.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "a,\
            \ndiv {\
            \n    top: 0;\
            \n}\
            \n\
            \n.a,\
            \n.b {\
            \n    &.c {\
            \n        color: red;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "a,\
        \ndiv {\
        \n  top: 0;\
        \n}\
        \n.a.c,\
        \n.b.c {\
        \n  color: red;\
        \n}\
        \n"
    );
}
