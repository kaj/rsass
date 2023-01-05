//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/unicode_variables.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unicode_variables")
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
