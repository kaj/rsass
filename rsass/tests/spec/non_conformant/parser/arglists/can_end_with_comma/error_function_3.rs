//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-3.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-function-3")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "// incorectly terminated\
             \n@function missing-paren-error($a,$b, {\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @function missing-paren-error($a,$b, {\
         \n  |                                      ^\
         \n  \'\
         \n  input.scss 2:38  root stylesheet",
    );
}
