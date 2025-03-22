//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/guard_assign.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("guard_assign")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: 2 !default;\n\
             \nfoo {a: $var}\n"),
        "foo {\
         \n  a: 2;\
         \n}\n"
    );
}
