//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-postfix-itpl")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{&}post {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}\r\n"),
        "post foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
