//! Tests auto-converted from "sass-spec/spec/directives/use/with/variable_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("variable_exists")
        .mock_file("_other.scss", "@use \"sass:meta\";\n$before-declaration: meta.variable-exists(a);\n$a: original !default;\nb {\n  before-declaration: $before-declaration;\n  after-declaration: meta.variable-exists(a);\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  before-declaration: false;\
         \n  after-declaration: true;\
         \n}\n"
    );
}
