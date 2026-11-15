//! Tests auto-converted from "sass-spec/spec/css/selector/pseudoselector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pseudoselector")
}

mod an_plus_b {
    use super::runner;

    mod escape {
        use super::runner;

        #[test]
        fn even() {
            assert_eq!(
                runner().ok(":nth-child(\\65 ven) {a: b}\n"),
                ":nth-child(even) {\
         \n  a: b;\
         \n}\n"
            );
        }
        #[test]
        fn n() {
            assert_eq!(
                runner().ok(":nth-child(1\\6e+2) {a: b}\n"),
                ":nth-child(1n+2) {\
         \n  a: b;\
         \n}\n"
            );
        }
        #[test]
        fn odd() {
            assert_eq!(
                runner().ok(":nth-child(\\6f dd) {a: b}\n"),
                ":nth-child(odd) {\
         \n  a: b;\
         \n}\n"
            );
        }
    }
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
