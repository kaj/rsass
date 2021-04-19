//! Tests auto-converted from "sass-spec/spec/css/supports/error.hrx"

mod syntax {
    mod anything {
        #[test]
        #[ignore] // wrong error
        fn colon() {
            assert_eq!(
                crate::rsass(
                    "@supports (a !:$) {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \":\".\
         \n  ,\
         \n1 | @supports (a !:$) {@b}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn non_identifier_start() {
            assert_eq!(
                crate::rsass(
                    "@supports (1 a) {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | @supports (1 a) {@b}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                crate::rsass(
                    "@supports (not a) {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (not a) {@b}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet\
         \n",
            );
        }
    }
    mod declaration {
        #[test]
        #[ignore] // wrong error
        fn multiple() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) (c: d) {@e}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @supports (a: b) (c: d) {@e}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                crate::rsass(
                    "@supports (not a: b) {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (not a: b) {@c}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn parens() {
            assert_eq!(
                crate::rsass(
                    "@supports ((a): b) {c}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \")\".\
         \n  ,\
         \n1 | @supports ((a): b) {c}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet\
         \n",
            );
        }
    }
    mod function {
        #[test]
        #[ignore] // wrong error
        fn not() {
            assert_eq!(
                crate::rsass(
                    "@supports not(:) {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | @supports not(:) {@b}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn space_before_arg() {
            assert_eq!(
                crate::rsass(
                    "@supports a (b) {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a (b) {@b}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
            );
        }
    }
    mod ident {
        #[test]
        #[ignore] // wrong error
        fn interpolated_after() {
            assert_eq!(
                crate::rsass(
                    "@supports a#{b} {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a#{b} {@c}\
         \n  |           ^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn interpolated_before() {
            assert_eq!(
                crate::rsass(
                    "@supports #{a}b {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports #{a}b {@c}\
         \n  |           ^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn plain() {
            assert_eq!(
                crate::rsass(
                    "@supports a {@b}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a {@b}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn ident_after_not() {
        assert_eq!(
            crate::rsass(
                "@supports not a {@b}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports not a {@b}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn none() {
        assert_eq!(
            crate::rsass(
                "@supports {@a}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports {@a}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
        );
    }
    mod operator {
        #[test]
        #[ignore] // wrong error
        fn and_after_not() {
            assert_eq!(
                crate::rsass(
                    "@supports not (a: b) and (c: d) {@e}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @supports not (a: b) and (c: d) {@e}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn lonely_not() {
            assert_eq!(
                crate::rsass(
                    "@supports not {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports not {@c}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not_after_and() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) and (not c: d) {@e}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports (a: b) and (not c: d) {@e}\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn not_function_after_and() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) and not() {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: \"not\" is not a valid identifier here.\
         \n  ,\
         \n1 | @supports (a: b) and not() {@c}\
         \n  |                      ^^^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn or_after_and() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) and (c: d) or (e: f) {@g}\
             \n"
                )
                .unwrap_err(),
                "Error: Expected \"and\".\
         \n  ,\
         \n1 | @supports (a: b) and (c: d) or (e: f) {@g}\
         \n  |                             ^\
         \n  \'\
         \n  input.scss 1:29  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn trailing_and() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) and {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports (a: b) and {@c}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn trailing_or() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) or {@c}\
             \n"
                )
                .unwrap_err(),
                "Error: expected \"(\".\
         \n  ,\
         \n1 | @supports (a: b) or {@c}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn raw_declaration() {
        assert_eq!(
            crate::rsass(
                "@supports a: b {@c}\
             \n"
            )
            .unwrap_err(),
            "Error: Expected @supports condition.\
         \n  ,\
         \n1 | @supports a: b {@c}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
        );
    }
}
