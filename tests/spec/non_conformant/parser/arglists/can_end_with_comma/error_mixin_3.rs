//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-3.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "// incorectly terminated\
             \n@mixin missing-paren-error($a,$b, {\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @mixin missing-paren-error($a,$b, {\
         \n  |                                   ^\
         \n  \'\
         \n  input.scss 2:35  root stylesheet\
         \n",
    );
}
