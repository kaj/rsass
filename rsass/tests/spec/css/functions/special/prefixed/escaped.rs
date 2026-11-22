//! Tests auto-converted from "sass-spec/spec/css/functions/special/prefixed/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaped")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$a: b;\
             \nc {d: -e-\\75 rl($a)}\n"),
        "c {\
         \n  d: -e-url(b);\
         \n}\n"
    );
}
