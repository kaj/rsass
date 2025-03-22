//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-prefix-itpl")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("pre#{&} {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}\r\n"),
        "pre foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
