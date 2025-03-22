//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse($selector: \"c\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
