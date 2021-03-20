//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_472.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  display: block;\
            \n  @keyframes {\
            \n    from {\
            \n      foo: bar;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  display: block;\
        \n}\
        \n@keyframes {\
        \n  from {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}
