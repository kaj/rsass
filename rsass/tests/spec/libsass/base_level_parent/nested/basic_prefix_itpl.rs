//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-prefix-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-prefix-itpl")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  pre#{&} {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "test pretest foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
