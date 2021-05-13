//! Tests auto-converted from "sass-spec/spec/css/supports/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod syntax {
    #[allow(unused)]
    use super::runner;
    mod anything {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn colon() {
            assert_eq!(
                runner().err("@supports (a !:$) {@b}\n"),
                "Error: expected \":\".\
         \n  ,\
         \n1 | @supports (a !:$) {@b}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn non_identifier_start() {
            assert_eq!(
                runner().err("@supports (1 a) {@b}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | @supports (1 a) {@b}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                runner().err("@supports (not a) {@b}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (not a) {@b}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
    }
    mod declaration {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn multiple() {
            assert_eq!(
                runner().err("@supports (a: b) (c: d) {@e}\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @supports (a: b) (c: d) {@e}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                runner().err("@supports (not a: b) {@c}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (not a: b) {@c}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn parens() {
            assert_eq!(
                runner().err("@supports ((a): b) {c}\n"),
                "Error: expected \")\".\
         \n  ,\
         \n1 | @supports ((a): b) {c}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
            );
        }
    }
    mod function {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                runner().err("@supports not(:) {@b}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | @supports not(:) {@b}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn space_before_arg() {
            assert_eq!(
                runner().err("@supports a (b) {@b}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a (b) {@b}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
    }
    mod ident {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn interpolated_after() {
            assert_eq!(
                runner().err("@supports a#{b} {@c}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a#{b} {@c}\
         \n  |           ^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn interpolated_before() {
            assert_eq!(
                runner().err("@supports #{a}b {@c}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports #{a}b {@c}\
         \n  |           ^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn plain() {
            assert_eq!(
                runner().err("@supports a {@b}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a {@b}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn ident_after_not() {
        assert_eq!(
            runner().err("@supports not a {@b}\n"),
            "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports not a {@b}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn none() {
        assert_eq!(
            runner().err("@supports {@a}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports {@a}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod operator {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn and_after_not() {
            assert_eq!(
                runner().err("@supports not (a: b) and (c: d) {@e}\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @supports not (a: b) and (c: d) {@e}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn lonely_not() {
            assert_eq!(
                runner().err("@supports not {@c}\n"),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports not {@c}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not_after_and() {
            assert_eq!(
                runner().err("@supports (a: b) and (not c: d) {@e}\n"),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (a: b) and (not c: d) {@e}\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not_function_after_and() {
            assert_eq!(
                runner().err("@supports (a: b) and not() {@c}\n"),
                "Error: \"not\" is not a valid identifier here.\
         \n  ,\
         \n1 | @supports (a: b) and not() {@c}\
         \n  |                      ^^^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn or_after_and() {
            assert_eq!(
                runner().err("@supports (a: b) and (c: d) or (e: f) {@g}\n"),
                "Error: Expected \"and\".\
         \n  ,\
         \n1 | @supports (a: b) and (c: d) or (e: f) {@g}\
         \n  |                             ^\
         \n  \'\
         \n  input.scss 1:29  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn trailing_and() {
            assert_eq!(
                runner().err("@supports (a: b) and {@c}\n"),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports (a: b) and {@c}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn trailing_or() {
            assert_eq!(
                runner().err("@supports (a: b) or {@c}\n"),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports (a: b) or {@c}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn raw_declaration() {
        assert_eq!(
            runner().err("@supports a: b {@c}\n"),
            "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a: b {@c}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
}
