//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_950.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".selector1{ foo: bar; }\
            \n.selector2{ zapf: dings; }\
            \n\
            \n.selector3{ @extend .selector1, .selector2; }"
        )
        .unwrap(),
        ".selector1, .selector3 {\
        \n  foo: bar;\
        \n}\
        \n.selector2, .selector3 {\
        \n  zapf: dings;\
        \n}\
        \n"
    );
}
