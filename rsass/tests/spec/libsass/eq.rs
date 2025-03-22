//! Tests auto-converted from "sass-spec/spec/libsass/eq.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("eq")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  foo: center == \"center\";\
             \n  foo: (a b c) == (a b c);\
             \n  foo: a b c == a b c;\
             \n}\n"),
        "div {\
         \n  foo: true;\
         \n  foo: true;\
         \n  foo: a b false b c;\
         \n}\n"
    );
}
