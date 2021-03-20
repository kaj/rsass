//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/escaped_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Escapes in selectors\' identifiers should be normalized before `@extend` is\
            \n// applied.\
            \n.foo {escape: none}\
            \n\\.foo {escape: slash dot}\
            \n\\2E foo {escape: hex}\
            \n\
            \n.bar {@extend \\02e foo}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  escape: none;\
        \n}\
        \n\\.foo, .bar {\
        \n  escape: slash dot;\
        \n}\
        \n\\.foo, .bar {\
        \n  escape: hex;\
        \n}\
        \n"
    );
}
