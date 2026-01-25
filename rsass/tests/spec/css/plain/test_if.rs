//! Tests auto-converted from "sass-spec/spec/css/plain/if.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("if")
        .mock_file("plain.css", "a {b: if(css(1): c; css(2): d; else: e)}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  b: if(css(1): c; css(2): d; else: e);\
         \n}\n"
    );
}
