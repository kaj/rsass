//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-prefix-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-prefix-itpl")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  @at-root {\r\
             \n    pre#{&} {\r\
             \n      foo {\r\
             \n        bar: baz;\r\
             \n      }\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "pretest foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
