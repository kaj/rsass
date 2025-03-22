//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-postfix-itpl")
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
