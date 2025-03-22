//! Tests auto-converted from "sass-spec/spec/css/selector/pseudoselector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pseudoselector")
}

mod error {
    use super::runner;

    mod with_attribute_mismatched {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn sass() {
            assert_eq!(
                runner().err(
                    "a:b([c)]{\
             \n  d: e;\
             \n}\n"
                ),
                "Error: expected \"]\".\
         \n  ,\
         \n1 | a:b([c)]{\
         \n  |       ^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod nested {
    use super::runner;

    #[test]
    fn adjacent_combinators() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#1038\
             \na {\
             \n  b:c, > d {x: y}\
             \n}\n"),
            "a b:c, a > d {\
         \n  x: y;\
         \n}\n"
        );
    }
}
