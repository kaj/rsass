//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/21_one_builtin_function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  color: rgb(255, $blue: 0, $green: 255);\
             \n  background: rgb(123, 45, 6);\
             \n}\n"),
        "div {\
         \n  color: rgb(255, 255, 0);\
         \n  background: rgb(123, 45, 6);\
         \n}\n"
    );
}
