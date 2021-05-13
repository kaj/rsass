//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested_rules_with_fancy_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  .bar {a: b}\
             \n  :baz {c: d}\
             \n  bang:bop {e: f}}\n"),
        "foo .bar {\
         \n  a: b;\
         \n}\
         \nfoo :baz {\
         \n  c: d;\
         \n}\
         \nfoo bang:bop {\
         \n  e: f;\
         \n}\n"
    );
}
