//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/00_empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("00_empty")
}

#[test]
fn test() {
    assert_eq!(runner().ok("\n"), "");
}
