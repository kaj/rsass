//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1794.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (max-width /*comment*/ : 500px) {\
            \n  foo { bar: baz; }\
            \n}"
        )
        .unwrap(),
        "@media (max-width: 500px) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}
