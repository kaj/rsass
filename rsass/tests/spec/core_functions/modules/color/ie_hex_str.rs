//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/ie_hex_str.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ie_hex_str")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.ie-hex-str(#abcdef)}\n"),
        "a {\
         \n  b: #FFABCDEF;\
         \n}\n"
    );
}
