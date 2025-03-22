//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/lang-bug.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lang-bug")
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
