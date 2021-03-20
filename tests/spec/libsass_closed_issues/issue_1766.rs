//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1766.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all { @import \"foo.scss\" }\
            \n@media all { @import \"foo.scss\"; }\
            \n"
        )
        .unwrap(),
        "@media all {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n@media all {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}
