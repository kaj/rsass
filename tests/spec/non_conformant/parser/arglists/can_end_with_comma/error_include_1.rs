//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "// double comma in middle of arglist\
             \n.error {\
             \n  @include double-comma-error($a,,$b);\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   @include double-comma-error($a,,$b);\
         \n  |                                  ^\
         \n  \'\
         \n  input.scss 3:34  root stylesheet",
    );
}
