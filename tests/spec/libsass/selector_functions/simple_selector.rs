//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/simple-selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo {\r\
             \n  test-01: simple-selectors(\".foo.bar\");\r\
             \n  test-02: simple-selectors(\".foo.bar.baz\");\r\
             \n}"),
        "foo {\
         \n  test-01: .foo, .bar;\
         \n  test-02: .foo, .bar, .baz;\
         \n}\n"
    );
}
