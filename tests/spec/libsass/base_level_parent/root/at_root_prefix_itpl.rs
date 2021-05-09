//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@at-root {\r\
             \n  pre#{&} {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "pre foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
