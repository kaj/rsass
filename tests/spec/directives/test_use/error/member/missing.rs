//! Tests auto-converted from "sass-spec/spec/directives/use/error/member/missing.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("missing")
        .mock_file("global/mixin/other.scss", "")
        .mock_file("global/variable/other.scss", "")
        .mock_file("namespaced/function/other.scss", "")
        .mock_file("namespaced/mixin/other.scss", "")
        .mock_file("namespaced/variable_declaration/other.scss", "")
        .mock_file("namespaced/variable_use/other.scss", "")
}

mod global {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("global")
    }

    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \n@include member;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include member;\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \na {b: $member}\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $member}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
mod namespaced {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("namespaced")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \na {b: other.member()}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n3 | a {b: other.member()}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \n@include other.member;\n"
            ),
            "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include other.member;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable_declaration() {
        let runner = runner().with_cwd("variable_declaration");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \nother.$member: value;\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn variable_use() {
        let runner = runner().with_cwd("variable_use");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \na {b: other.$member}\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: other.$member}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
}
