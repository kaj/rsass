//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/ie_hex_str.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ie_hex_str")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str(#abcdef)}\n"),
        "a {\
         \n  b: #FFABCDEF;\
         \n}\n"
    );
}
