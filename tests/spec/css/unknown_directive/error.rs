//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/error.hrx"

#[test]
#[ignore] // wrong error
fn in_declaration() {
    assert_eq!(
        crate::rsass(
            "// Unknown at-rules aren\'t allowed in property declarations.\
             \n.x {\
             \n  y: {\
             \n    @asdf;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n4 |     @asdf;\
         \n  |     ^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn in_function() {
    assert_eq!(
        crate::rsass(
            "// Unknown at-rules aren\'t allowed in functions.\
             \n@function foo() {\
             \n  @asdf;\
             \n  @return null;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |   @asdf;\
         \n  |   ^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet\
         \n",
    );
}
mod interpolation {
    #[test]
    #[ignore] // wrong error
    fn in_declaration() {
        assert_eq!(
        crate::rsass(
            "// Unknown at-rules aren\'t allowed in property declarations.\
             \n.x {\
             \n  y: {\
             \n    @#{\"asdf\"};\
             \n  }\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n4 |     @#{\"asdf\"};\
         \n  |      ^\
         \n  \'\
         \n  input.scss 4:6  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn in_function() {
        assert_eq!(
            crate::rsass(
                "// Unknown at-rules aren\'t allowed in functions.\
             \n@function foo() {\
             \n  @#{\"asdf\"};\
             \n  @return null;\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected identifier.\
         \n  ,\
         \n3 |   @#{\"asdf\"};\
         \n  |    ^\
         \n  \'\
         \n  input.scss 3:4  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space_after_at() {
        assert_eq!(
        crate::rsass(
            "// No whitespace is allowed between the @ and the rule name.\
             \n@ #{\"unknown\"};\
             \n"
        ).unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n2 | @ #{\"unknown\"};\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet\
         \n",
    );
    }
}
#[test]
#[ignore] // wrong error
fn space_after_at() {
    assert_eq!(
        crate::rsass(
            "// No whitespace is allowed between the @ and the rule name.\
             \n@ unknown;\
             \n"
        )
        .unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n2 | @ unknown;\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet\
         \n",
    );
}
