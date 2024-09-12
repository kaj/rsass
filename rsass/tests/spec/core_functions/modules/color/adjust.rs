//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/adjust.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("adjust")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: 10)}\n"),
        "a {\
         \n  b: #b5cdef;\
         \n}\n"
    );
}
