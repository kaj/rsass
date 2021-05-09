//! Tests auto-converted from "sass-spec/spec/css/empty_block_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("@foo {}\n"), "@foo {}\n");
}
