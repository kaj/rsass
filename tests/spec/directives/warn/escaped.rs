//! Tests auto-converted from "sass-spec/spec/directives/warn/escaped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@w\\61rn warning;\
             \na {b: c}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
