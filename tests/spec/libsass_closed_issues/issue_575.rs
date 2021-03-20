//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_575.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\
            \n  @if (foo: bar) == (foo: bar) {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
