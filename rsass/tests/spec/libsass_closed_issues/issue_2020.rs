//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2020")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\n\
             \nform {\
             \n  $selector: list.nth(&, 1);\
             \n  sel: meta.inspect($selector);\
             \n  $selector: list.nth($selector, 1);\
             \n  sel: meta.inspect($selector);\
             \n}"),
        "form {\
         \n  sel: form;\
         \n  sel: form;\
         \n}\n"
    );
}
