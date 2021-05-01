//! Tests auto-converted from "sass-spec/spec/directives/use/error/load.hrx"

mod conflict {
    #[test]
    #[ignore] // wrong error
    fn all() {
        assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it has conflicting partials *and*\
             \n// conflicting extensions.\
             \n@use \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.sass\
         \n  other.sass\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
    mod extension {
        #[test]
        #[ignore] // wrong error
        fn sass_and_scss() {
            assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the \".sass\" or\
             \n// \".scss\" file.\
             \n@use \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.sass\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn index() {
        assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial index file.\
             \n@use \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  other/_index.scss\
         \n  other/index.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn partial() {
        assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial file.\
             \n@use \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
}
mod conflicting_namespace {
    #[test]
    #[ignore] // missing error
    fn built_in() {
        assert_eq!(
            crate::rsass(
                "// Regression test for sass/dart-sass#1047\
             \n@use \"sass:math\";\
             \n@use \"sass:math\";\
             \n"
            )
            .unwrap_err(),
            "Error: There\'s already a module with namespace \"math\".\
         \n  ,\
         \n2 | @use \"sass:math\";\
         \n  | ================ original @use\
         \n3 | @use \"sass:math\";\
         \n  | ^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn explicit() {
        assert_eq!(
            crate::rsass(
                "@use \"other1\" as other;\
             \n@use \"other2\" as other;\
             \n"
            )
            .unwrap_err(),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"other1\" as other;\
         \n  | ====================== original @use\
         \n2 | @use \"other2\" as other;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn implicit() {
        assert_eq!(
            crate::rsass(
                "@use \"dir1/other\";\
             \n@use \"dir2/other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"dir1/other\";\
         \n  | ================= original @use\
         \n2 | @use \"dir2/other\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn mixed() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n@use \"other2\" as other;\
             \n"
            )
            .unwrap_err(),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ============ original @use\
         \n2 | @use \"other2\" as other;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn dir_dot_scss() {
    assert_eq!(
        crate::rsass(
            "@use \"dir.scss\";\
             \n"
        )
        .unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"dir.scss\";\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod test_loop {
    #[test]
    #[ignore] // missing error
    fn import_to_use() {
        assert_eq!(
            crate::rsass(
                "@import \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn use_self() {
        assert_eq!(
            crate::rsass(
                "@use \"input\";\
             \n"
            )
            .unwrap_err(),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn use_to_import() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: This file is already being loaded.\
         \n  ,\
         \n1 | @import \"input\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  other.scss 1:9  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn use_to_use() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn missing() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
             \n"
        )
        .unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn no_extension() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
             \n"
        )
        .unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn unknown_scheme() {
    assert_eq!(
        crate::rsass(
            "@use \"scheme:bar\";\
             \n"
        )
        .unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"scheme:bar\";\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
