//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/08_selector_combinators.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("08_selector_combinators")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a   +   b  >  c {\
             \n  d e {\
             \n    color: blue;\
             \n    background: white;\
             \n  }\
             \n  color: red;\
             \n  background: gray;\
             \n}"),
        "a + b > c {\
         \n  color: red;\
         \n  background: gray;\
         \n}\
         \na + b > c d e {\
         \n  color: blue;\
         \n  background: white;\
         \n}\n"
    );
}
