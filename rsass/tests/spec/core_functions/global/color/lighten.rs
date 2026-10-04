//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/lighten.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lighten")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: lighten(#abcdef, 10%)}\n"),
        "a {\
         \n  b: rgb(83.8588235294%, 90.3921568627%, 96.9254901961%);\
         \n}\n"
    );
}
