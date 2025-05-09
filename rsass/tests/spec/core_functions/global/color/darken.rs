//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/darken.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("darken")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: darken(#abcdef, 10%)}\n"),
        "a {\
         \n  b: rgb(128.16, 179.5, 230.84);\
         \n}\n"
    );
}
