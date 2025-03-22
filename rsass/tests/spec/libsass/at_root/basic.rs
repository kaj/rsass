//! Tests auto-converted from "sass-spec/spec/libsass/at-root/basic.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  color: blue;\n\
             \n  @at-root {\
             \n    bar {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  color: blue;\n\
             \n  @at-root bar {\
             \n    color: red;\
             \n  }\
             \n}\n"),
        "foo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\n"
    );
}
