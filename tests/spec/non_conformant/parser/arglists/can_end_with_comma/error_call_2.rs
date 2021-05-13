//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-2.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "// double comma at end of arglist\
             \n.error {\
             \n  a: double-comma-error($a,$b,,);\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   a: double-comma-error($a,$b,,);\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 3:31  root stylesheet",
    );
}
