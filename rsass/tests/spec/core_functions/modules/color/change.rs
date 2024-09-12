//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/change.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("change")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(#abcdef, $red: 10)}\n"),
        "a {\
         \n  b: #0acdef;\
         \n}\n"
    );
}
