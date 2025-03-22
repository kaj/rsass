//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/003_test_variables.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("003_test_variables")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: foo;\n\
             \nblat {a: $var}\n"),
        "blat {\
         \n  a: foo;\
         \n}\n"
    );
}
