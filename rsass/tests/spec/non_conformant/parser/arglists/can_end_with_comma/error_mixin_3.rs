//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-3.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-mixin-3")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "// incorectly terminated\
             \n@mixin missing-paren-error($a,$b, {\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @mixin missing-paren-error($a,$b, {\
         \n  |                                   ^\
         \n  \'\
         \n  input.scss 2:35  root stylesheet",
    );
}
