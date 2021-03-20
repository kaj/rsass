//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1398.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen and (hux: 3/4) {\
            \n  foo {\
            \n    bar: baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (hux: 3/4) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}
