//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-debug/simple.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@debug \"debug\";"), "");
}
