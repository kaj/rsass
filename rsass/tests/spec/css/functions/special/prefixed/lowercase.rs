//! Tests auto-converted from "sass-spec/spec/css/functions/special/prefixed/lowercase.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lowercase")
}

mod calc {
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -a-calc(#{0})}\n"),
            "a {\
         \n  b: -a-calc(0);\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: -a-calc(0)}\n"),
            "a {\
         \n  b: -a-calc(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok(
                "a {b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -a-calc($c)}\n"),
            "a {\
         \n  b: -a-calc($c);\
         \n}\n"
        );
    }
}
mod element {
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -c-element(#{0})}\n"),
            "a {\
         \n  b: -c-element(0);\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: -c-element(0)}\n"),
            "a {\
         \n  b: -c-element(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok(
                "a {b: -c-element(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -c-element(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -c-element($d)}\n"),
            "a {\
         \n  b: -c-element($d);\
         \n}\n"
        );
    }
}
mod expression {
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -c-expression(#{0})}\n"),
            "a {\
         \n  b: -c-expression(0);\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: -c-expression(0)}\n"),
            "a {\
         \n  b: -c-expression(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
        runner().ok(
            "a {b: -c-expression(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
        ),
        "a {\
         \n  b: -c-expression(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
    );
    }
    #[test]
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -c-expression($d)}\n"),
            "a {\
         \n  b: -c-expression($d);\
         \n}\n"
        );
    }
}
mod progid {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -c-progid:d(#{0})}\n"),
            "a {\
         \n  b: -c-progid:d(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -c-progid:d(0)}\n"),
            "a {\
         \n  b: -c-progid:d(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok(
                "a {b: -c-progid:d(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -c-progid:d(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -c-progid:d($d)}\n"),
            "a {\
         \n  b: -c-progid:d($d);\
         \n}\n"
        );
    }
}
mod url {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -c-url(#{0})}\n"),
            "a {\
         \n  b: url(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -c-url(0)}\n"),
            "a {\
         \n  b: url(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok("a {b: -c-url(http://d.e/f!g)}\n"),
            "a {\
         \n  b: url(http://d.e/f!g);\
         \n}\n"
        );
    }
    #[test]
    fn script_like() {
        assert_eq!(
            runner().ok("$a: b;\
             \nc {d: -e-url($a)}\n"),
            "c {\
         \n  d: -e-url(b);\
         \n}\n"
        );
    }
}
