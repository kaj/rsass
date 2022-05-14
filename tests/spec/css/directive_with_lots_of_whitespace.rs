//! Tests auto-converted from "sass-spec/spec/css/directive_with_lots_of_whitespace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("directive_with_lots_of_whitespace")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@foo \"bar\";\n"), "@foo \"bar\";\n");
}
