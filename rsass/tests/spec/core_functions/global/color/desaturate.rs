//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/desaturate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("desaturate")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: desaturate(#abcdef, 10%)}\n"),
        "a {\
         \n  b: #b0cdea;\
         \n}\n"
    );
}
