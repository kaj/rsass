//! Tests auto-converted from "sass-spec/spec/directives/import/error/conflict.hrx"

#[test]
#[ignore] // missing error
fn all() {
    assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it has conflicting partials *and*\
             \n// conflicting extensions.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.sass\
         \n  other.sass\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn extension() {
    assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the \".sass\" or\
             \n// \".scss\" file.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.sass\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
mod import_only {
    #[test]
    #[ignore] // missing error
    fn no_extension() {
        assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the Sass or\
             \n// the SCSS import-only file.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.import.sass\
         \n  other.import.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn with_extension() {
        assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial.\
             \n@import \"other.scss\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.import.scss\
         \n  other.import.scss\
         \n  ,\
         \n3 | @import \"other.scss\";\
         \n  |         ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn index() {
    assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial index file.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  other/_index.scss\
         \n  other/index.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn partial() {
    assert_eq!(
        crate::rsass(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial file.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
