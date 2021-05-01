//! Tests auto-converted from "sass-spec/spec/directives/use/error/with.hrx"

#[test]
#[ignore] // wrong error
fn conflict() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\" with ($a: b);\
             \n"
        )
        .unwrap_err(),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"left\" as *;\
         \n    | ================ includes variable\
         \n2   | @use \"right\" as *;\
         \n    | ================= includes variable\
         \n... |\
         \n4   | $a: c !default;\
         \n    | ^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  _midstream.scss 4:1  @use\
         \n  input.scss 1:1       root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn core_module() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\" with ($a: b);\
             \n"
        )
        .unwrap_err(),
        "Error: Built-in modules can\'t be configured.\
         \n  ,\
         \n1 | @use \"sass:color\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod invalid_expression {
    #[test]
    #[ignore] // wrong error
    fn error() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" with ($a: 1px + 1em);\
             \n"
            )
            .unwrap_err(),
            "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | @use \"other\" with ($a: 1px + 1em);\
         \n  |                        ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn module_loaded_later() {
        assert_eq!(
            crate::rsass(
                "@use \"configured\" with ($a: other.$b);\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | @use \"configured\" with ($a: other.$b);\
         \n  |                             ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:29  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn variable_defined_later() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" with ($a: $b);\
             \n$b: c;\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined variable.\
         \n  ,\
         \n1 | @use \"other\" with ($a: $b);\
         \n  |                        ^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
}
mod multi_configuration {
    #[test]
    #[ignore] // wrong error
    fn multi_file() {
        assert_eq!(
        crate::rsass(
            "@use \"left\";\
             \n@use \"right\";\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _right.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _left.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n  \'\
         \n  _right.scss 1:1  @use\
         \n  input.scss 2:1   root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn one_file() {
        assert_eq!(
        crate::rsass(
            "@use \"other\" as o1 with ($a: b);\
             \n@use \"other\" as o2 with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1 with ($a: b);\
         \n  | =============================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn through_forward() {
        assert_eq!(
        crate::rsass(
            "@use \"forwarded\";\
             \n@use \"midstream\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n1 | @use \"forwarded\";\
         \n  | ================ original load\
         \n2 | @use \"midstream\" with ($a: b);\
         \n  | ============================= configuration\
         \n  \'\
         \n  _midstream.scss 1:1  @use\
         \n  input.scss 2:1       root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unconfigured_first() {
        assert_eq!(
        crate::rsass(
            "@use \"other\" as o1;\
             \n@use \"other\" as o2 with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1;\
         \n  | ================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn namespace() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"midstream\" with ($a: b);\
         \n  |                        ^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn nested() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn not_default() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn repeated_variable() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: b, $a: c);\
             \n"
        )
        .unwrap_err(),
        "Error: The same variable may only be configured once.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b, $a: c);\
         \n  |                           ^^^^^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
mod through_forward {
    #[test]
    #[ignore] // wrong error
    fn test_as() {
        assert_eq!(
        crate::rsass(
            "@use \"used\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn hide() {
        assert_eq!(
        crate::rsass(
            "@use \"used\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn show() {
        assert_eq!(
        crate::rsass(
            "@use \"used\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn with() {
        assert_eq!(
        crate::rsass(
            "@use \"used\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn undefined() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
