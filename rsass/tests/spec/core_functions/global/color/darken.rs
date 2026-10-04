//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/darken.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("darken")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {b: darken(#abcdef, 10%)}\n"),
        "a {\
         \n  b: rgb(50.2588235294%, 70.3921568627%, 90.5254901961%);\
         \n}\n"
    );
}
