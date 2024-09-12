//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/simple-selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple-selector")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \nfoo {\r\
             \n  test-01: selector.simple-selectors(\".foo.bar\");\r\
             \n  test-02: selector.simple-selectors(\".foo.bar.baz\");\r\
             \n}"),
        "foo {\
         \n  test-01: .foo, .bar;\
         \n  test-02: .foo, .bar, .baz;\
         \n}\n"
    );
}
