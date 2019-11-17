//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_83.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin colors($color) {\r\
             \n  border-color: $color;\r\
             \n  background-color: $color;\r\
             \n  @content;\r\
             \n}\r\
             \n.colors {\r\
             \n  @include colors {\r\
             \n    color: $color;\r\
             \n  }\r\
             \n  border-width: 10px;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Missing argument $color.\
         \n    ,\
         \n1   | @mixin colors($color) {\
         \n    |        ============== declaration\
         \n... |\
         \n7   |   @include colors {\
         \n    |   ^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 7:3  colors()\
         \n  input.scss 7:3  root stylesheet\
         \n",
    );
}
