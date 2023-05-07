//! Tests auto-converted from "sass-spec/spec/css/media/logic/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod and_after {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn or() {
        assert_eq!(
            runner().err("@media (a) or (b) and (c) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media (a) or (b) and (c) {x {y: z}}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn type_and_not() {
        assert_eq!(
            runner().err("@media a and not (b) and (c) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and not (b) and (c) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
}
mod missing_whitespace {
    #[allow(unused)]
    use super::runner;

    mod and {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn after_type() {
            assert_eq!(
                runner().err("@media a and(b) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media a and(b) {x {y: z}}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn first() {
            assert_eq!(
                runner().err("@media (a) and(b) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) and(b) {x {y: z}}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn later() {
            assert_eq!(
                runner().err("@media (a) and (b) and(c) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) and (b) and(c) {x {y: z}}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
            );
        }
    }
    mod and_not {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
                runner().err("@media a and not(b) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media a and not(b) {x {y: z}}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn type_and_modifier() {
            assert_eq!(
                runner().err("@media only a and not(b) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media only a and not(b) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("@media not(a) {x {y: z}}\n"),
            "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media not(a) {x {y: z}}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod or {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn first() {
            assert_eq!(
                runner().err("@media (a) or(b) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) or(b) {x {y: z}}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn later() {
            assert_eq!(
                runner().err("@media (a) or (b) or(c) {x {y: z}}\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) or (b) or(c) {x {y: z}}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
            );
        }
    }
}
mod nothing_after {
    #[allow(unused)]
    use super::runner;

    mod and {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn after_interpolation() {
            assert_eq!(
                runner().err("@media #{a} and {x {y: z}}\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media #{a} and {x {y: z}}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn after_paren() {
            assert_eq!(
                runner().err("@media (a) and {x {y: z}}\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media (a) and {x {y: z}}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn after_type() {
            assert_eq!(
                runner().err("@media a and {x {y: z}}\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media a and {x {y: z}}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn and_not() {
        assert_eq!(
            runner().err("@media a and not {x {y: z}}\n"),
            "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media a and not {x {y: z}}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("@media not {x {y: z}}\n"),
            "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media not {x {y: z}}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn or() {
        assert_eq!(
            runner().err("@media (a) or {x {y: z}}\n"),
            "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media (a) or {x {y: z}}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
}
mod or_after {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn and() {
        assert_eq!(
            runner().err("@media (a) and (b) or (c) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media (a) and (b) or (c) {x {y: z}}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn interpolation() {
        assert_eq!(
            runner().err("@media #{\"(a)\"} or (b) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media #{\"(a)\"} or (b) {x {y: z}}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            runner().err("@media a or (b) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a or (b) {x {y: z}}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn type_and_not() {
        assert_eq!(
            runner().err("@media a and not (b) or (c) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and not (b) or (c) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn type_then_and() {
        assert_eq!(
            runner().err("@media a and (b) or (c) {x {y: z}}\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and (b) or (c) {x {y: z}}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
}
