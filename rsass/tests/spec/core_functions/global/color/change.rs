//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/change.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("change")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: change-color(#abcdef, $red: 10)}\n"),
        "a {\
         \n  b: #0acdef;\
         \n}\n"
    );
}
