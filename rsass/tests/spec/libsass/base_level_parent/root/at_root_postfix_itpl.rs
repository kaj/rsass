//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-postfix-itpl")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@at-root {\r\
             \n  #{&}post {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "post foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
