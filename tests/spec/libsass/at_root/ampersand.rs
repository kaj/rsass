//! Tests auto-converted from "sass-spec/spec/libsass/at-root/ampersand.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ampersand")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @at-root {\
             \n    & {\
             \n      color: blue;\
             \n    }\n\
             \n    &--modifier {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\n"),
        "foo {\
         \n  color: blue;\
         \n}\
         \nfoo--modifier {\
         \n  color: red;\
         \n}\n"
    );
}
