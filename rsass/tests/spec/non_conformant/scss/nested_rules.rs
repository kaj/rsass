//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested_rules.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested_rules")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar {baz {a: b}}\
             \n  bang {bip {a: b}}}\n"),
        "foo bar baz {\
         \n  a: b;\
         \n}\
         \nfoo bang bip {\
         \n  a: b;\
         \n}\n"
    );
}
