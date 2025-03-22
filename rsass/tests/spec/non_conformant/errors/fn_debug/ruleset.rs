//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug/ruleset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ruleset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\r\
             \n  @debug \"debug\";\r\
             \n  foo: bar;\r\
             \n}"),
        "a {\
         \n  foo: bar;\
         \n}\n"
    );
}
