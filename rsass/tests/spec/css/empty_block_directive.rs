//! Tests auto-converted from "sass-spec/spec/css/empty_block_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty_block_directive")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@foo {}\n"), "@foo {}\n");
}
