//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/only_one_bogus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("only_one_bogus")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a, + b {c: d}\n"),
        "a, + b {\
         \n  c: d;\
         \n}\n"
    );
}
