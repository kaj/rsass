//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/006_test_guard_assign.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("006_test_guard_assign")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: 1;\
             \n$var: 2 !default;\n\
             \nfoo {a: $var}\n"),
        "foo {\
         \n  a: 1;\
         \n}\n"
    );
}
