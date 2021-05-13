//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("@warn \"warn\";"), "");
}
