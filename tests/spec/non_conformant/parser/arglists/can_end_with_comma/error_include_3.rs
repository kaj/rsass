//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-include-3")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "// incorectly terminated\
             \n.error {\
             \n  @include incorrectly-terminated($a,$b,;\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   @include incorrectly-terminated($a,$b,;\
         \n  |                                         ^\
         \n  \'\
         \n  input.scss 3:41  root stylesheet",
    );
}
