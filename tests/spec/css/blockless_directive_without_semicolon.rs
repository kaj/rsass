//! Tests auto-converted from "sass-spec/spec/css/blockless_directive_without_semicolon.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("@foo \"bar\";\n"), "@foo \"bar\";\n");
}
