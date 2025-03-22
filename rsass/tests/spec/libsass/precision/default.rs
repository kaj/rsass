//! Tests auto-converted from "sass-spec/spec/libsass/precision/default.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("default")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \ntest {\r\
             \n  foo: 0.4999 math.round(0.4999);\r\
             \n  bar: 0.49999 math.round(0.49999);\r\
             \n  baz: 0.499999 math.round(0.499999);\r\
             \n  baz: 0.49999999999 math.round(0.49999999999);\r\
             \n}\r\n"),
        "test {\
         \n  foo: 0.4999 0;\
         \n  bar: 0.49999 0;\
         \n  baz: 0.499999 0;\
         \n  baz: 0.5 0;\
         \n}\n"
    );
}
