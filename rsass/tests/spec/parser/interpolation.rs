//! Tests auto-converted from "sass-spec/spec/parser/interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolation")
}

mod error {
    use super::runner;

    mod partial_bracket {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn scss() {
            assert_eq!(
                runner().err(
                    "[a#{\"]:is(b\"}) {c:d}\
             \n "
                ),
                "Error: expected \"]\".\
         \n  ,\
         \n1 | [a#{\"]:is(b\"}) {c:d}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
    }
}
