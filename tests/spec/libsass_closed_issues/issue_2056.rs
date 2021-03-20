//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2056.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ":foobar(.baz) {\
            \n    color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        ":foobar(.baz) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
