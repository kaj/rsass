//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1685.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($x, $y...) { @return null }\
            \n\
            \na {\
            \n  b: foo(1 2 3...);\
            \n}"
        )
        .unwrap(),
        ""
    );
}
