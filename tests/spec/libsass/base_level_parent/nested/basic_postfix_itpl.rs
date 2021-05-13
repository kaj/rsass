//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  #{&}post {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "test testpost foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
