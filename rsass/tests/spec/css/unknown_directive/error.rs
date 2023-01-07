//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
fn in_declaration() {
    assert_eq!(
        runner().err(
            "// Unknown at-rules aren\'t allowed in property declarations.\
             \n.x {\
             \n  y: {\
             \n    @asdf;\
             \n  }\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n4 |     @asdf;\
         \n  |     ^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
    );
}
#[test]
fn in_function() {
    assert_eq!(
        runner().err(
            "// Unknown at-rules aren\'t allowed in functions.\
             \n@function foo() {\
             \n  @asdf;\
             \n  @return null;\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |   @asdf;\
         \n  |   ^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
mod interpolation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn in_declaration() {
        assert_eq!(
        runner().err(
            "// Unknown at-rules aren\'t allowed in property declarations.\
             \n.x {\
             \n  y: {\
             \n    @#{\"asdf\"};\
             \n  }\
             \n}\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n4 |     @#{\"asdf\"};\
         \n  |      ^\
         \n  \'\
         \n  input.scss 4:6  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn in_function() {
        assert_eq!(
            runner().err(
                "// Unknown at-rules aren\'t allowed in functions.\
             \n@function foo() {\
             \n  @#{\"asdf\"};\
             \n  @return null;\
             \n}\n"
            ),
            "Error: Expected identifier.\
         \n  ,\
         \n3 |   @#{\"asdf\"};\
         \n  |    ^\
         \n  \'\
         \n  input.scss 3:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn space_after_at() {
        assert_eq!(
        runner().err(
            "// No whitespace is allowed between the @ and the rule name.\
             \n@ #{\"unknown\"};\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n2 | @ #{\"unknown\"};\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn space_after_at() {
    assert_eq!(
        runner().err(
            "// No whitespace is allowed between the @ and the rule name.\
             \n@ unknown;\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n2 | @ unknown;\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet",
    );
}
