//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-2.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-function-2")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "// double comma at end of arglist\
             \n@function double-comma-error($a,$b,,) {\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @function double-comma-error($a,$b,,) {\
         \n  |                                    ^\
         \n  \'\
         \n  input.scss 2:36  root stylesheet",
    );
}
