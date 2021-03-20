//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415/variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$parent: &;\
            \n\
            \n@if $parent {\
            \n  foo {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
