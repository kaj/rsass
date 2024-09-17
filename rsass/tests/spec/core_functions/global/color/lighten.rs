//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/lighten.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lighten")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: lighten(#abcdef, 10%)}\n"),
        "a {\
         \n  b: rgb(213.84, 230.5, 247.16);\
         \n}\n"
    );
}
