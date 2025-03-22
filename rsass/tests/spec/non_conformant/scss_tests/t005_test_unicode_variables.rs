//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/005_test_unicode_variables.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("005_test_unicode_variables")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$vär: foo;\n\
             \nblat {a: $vär}\n"),
        "blat {\
         \n  a: foo;\
         \n}\n"
    );
}
