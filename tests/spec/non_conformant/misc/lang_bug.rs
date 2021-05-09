//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/lang-bug.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div:lang(nb) {\
             \n  color: red;\
             \n}"),
        "div:lang(nb) {\
         \n  color: red;\
         \n}\n"
    );
}
