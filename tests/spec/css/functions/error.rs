//! Tests auto-converted from "sass-spec/spec/css/functions/error.hrx"

mod single_equals {
    #[test]
    #[ignore] // missing error
    fn no_lhs() {
        assert_eq!(
            crate::rsass(
                "a {b: c(=d)}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: c(=d)}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn no_lhs_or_rhs() {
        assert_eq!(
            crate::rsass(
                "a {b: c(=)}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: c(=)}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn no_rhs() {
        assert_eq!(
            crate::rsass(
                "a {b: c(d=)}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: c(d=)}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
}
