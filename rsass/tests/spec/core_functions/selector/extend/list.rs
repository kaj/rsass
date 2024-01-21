//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("list")
}

#[test]
#[ignore] // wrong result
fn all_match() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\".c.d\", \".c, .d\", \".e\")}\n"),
        "a {\
         \n  b: .c.d, .e;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn different_matches() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\".c.d, .c .e, .d .f\", \".c, .d\", \".g\")}\n"
        ),
        "a {\
         \n  b: .c.d, .g, .c .e, .g .e, .d .f, .g .f;\
         \n}\n"
    );
}
#[test]
fn one_matches() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\".c\", \".c, .d\", \".e\")}\n"),
        "a {\
         \n  b: .c, .e;\
         \n}\n"
    );
}
