//! Tests auto-converted from "sass-spec/spec/parser/operator_precedence.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("operator_precedence")
}

#[test]
fn mixed() {
    assert_eq!(
        runner().ok("// Regression test for scssphp/scssphp#435\
             \na {\
             \n  b: true or 1 < 0 and false;\
             \n}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
