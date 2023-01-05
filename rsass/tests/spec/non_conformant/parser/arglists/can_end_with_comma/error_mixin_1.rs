//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-mixin-1")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "// double comma in middle of arglist\
             \n@mixin double-comma-error($a,,$b) {\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @mixin double-comma-error($a,,$b) {\
         \n  |                              ^\
         \n  \'\
         \n  input.scss 2:30  root stylesheet",
    );
}
