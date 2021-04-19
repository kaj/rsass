//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-2.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "// double comma at end of arglist\
             \n.error {\
             \n  @include double-comma-error($a,$b,,);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   @include double-comma-error($a,$b,,);\
         \n  |                                     ^\
         \n  \'\
         \n  input.scss 3:37  root stylesheet\
         \n",
    );
}
