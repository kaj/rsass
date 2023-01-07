//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/03_simple_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_simple_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$color: red;\n\
             \na {\
             \n  color: $color;\
             \n}"),
        "a {\
         \n  color: red;\
         \n}\n"
    );
}
