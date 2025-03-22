//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/classes-and-ids.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("classes-and-ids")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div.foo {\
             \n  color: red;\
             \n  #hux buz {\
             \n    width: auto;\
             \n  }\
             \n  > .mux {\
             \n    text-align: center;\
             \n  }\
             \n}"),
        "div.foo {\
         \n  color: red;\
         \n}\
         \ndiv.foo #hux buz {\
         \n  width: auto;\
         \n}\
         \ndiv.foo > .mux {\
         \n  text-align: center;\
         \n}\n"
    );
}
