//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn/ruleset.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\r\
             \n  @warn \"warn\";\r\
             \n  foo: bar;\r\
             \n}"),
        "a {\
         \n  foo: bar;\
         \n}\n"
    );
}
