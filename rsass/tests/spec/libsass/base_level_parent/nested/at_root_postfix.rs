//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-postfix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-postfix")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  @at-root {\r\
             \n    &post {\r\
             \n      foo {\r\
             \n        bar: baz;\r\
             \n      }\r\
             \n    }\r\
             \n  }\r\
             \n}"),
        "testpost foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
