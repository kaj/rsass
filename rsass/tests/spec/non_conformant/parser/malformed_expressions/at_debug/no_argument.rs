//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-debug/no-argument.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no-argument")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@debug;\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @debug;\
         \n  |       ^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
