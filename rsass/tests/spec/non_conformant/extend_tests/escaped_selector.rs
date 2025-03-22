//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/escaped_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaped_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "// Escapes in selectors\' identifiers should be normalized before `@extend` is\
             \n// applied.\
             \n.foo {escape: none}\
             \n\\.foo {escape: slash dot}\
             \n\\2E foo {escape: hex}\n\
             \n.bar {@extend \\02e foo}\n"
        ),
        ".foo {\
         \n  escape: none;\
         \n}\
         \n\\.foo, .bar {\
         \n  escape: slash dot;\
         \n}\
         \n\\.foo, .bar {\
         \n  escape: hex;\
         \n}\n"
    );
}
