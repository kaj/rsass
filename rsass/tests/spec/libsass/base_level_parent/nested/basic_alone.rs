//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-alone.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  & {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}"),
        "test foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
