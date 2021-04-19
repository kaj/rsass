//! Tests auto-converted from "sass-spec/spec/expressions/syntax.hrx"

mod error {
    #[test]
    #[ignore] // missing error
    fn single_dot() {
        assert_eq!(
            crate::rsass(
                "a {a: .}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {a: .}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet\
         \n",
        );
    }
}
