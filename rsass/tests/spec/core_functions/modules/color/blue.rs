//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/blue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("blue")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue(#abcdef)}\n"),
        "a {\
         \n  b: 239;\
         \n}\n"
    );
}
