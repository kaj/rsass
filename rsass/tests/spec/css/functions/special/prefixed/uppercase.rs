//! Tests auto-converted from "sass-spec/spec/css/functions/special/prefixed/uppercase.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("uppercase")
}

mod calc {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -A-CALC(#{0})}\n"),
            "a {\
         \n  b: -a-calc(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -A-CALC(0)}\n"),
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
                "a {b: -A-CALC(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -A-CALC($c)}\n"),
            "a {\
         \n  b: -a-calc($c);\
         \n}\n"
        );
    }
}
mod element {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -C-ELEMENT(#{0})}\n"),
            "a {\
         \n  b: -c-element(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -C-ELEMENT(0)}\n"),
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
                "a {b: -C-ELEMENT(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -c-element(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -C-ELEMENT($d)}\n"),
            "a {\
         \n  b: -c-element($d);\
         \n}\n"
        );
    }
}
mod expression {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: -C-EXPRESSION(#{0})}\n"),
            "a {\
         \n  b: -c-expression(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -C-EXPRESSION(0)}\n"),
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
            "a {b: -C-EXPRESSION(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
        ),
        "a {\
         \n  b: -c-expression(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -C-EXPRESSION($d)}\n"),
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
            runner().ok("a {b: -C-PROGID:D(#{0})}\n"),
            "a {\
         \n  b: -c-progid:D(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -C-PROGID:D(0)}\n"),
            "a {\
         \n  b: -c-progid:D(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok(
                "a {b: -C-PROGID:D(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
            ),
            "a {\
         \n  b: -c-progid:D(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn script_like() {
        assert_eq!(
            runner().ok("a {b: -C-PROGID:D($d)}\n"),
            "a {\
         \n  b: -c-progid:D($d);\
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
            runner().ok("a {b: -C-URL(#{0})}\n"),
            "a {\
         \n  b: url(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            runner().ok("a {b: -C-URL(0)}\n"),
            "a {\
         \n  b: url(0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner().ok("a {b: -C-URL(http://d.e/f!g)}\n"),
            "a {\
         \n  b: url(http://d.e/f!g);\
         \n}\n"
        );
    }
    #[test]
    fn script_like() {
        assert_eq!(
            runner().ok("$a: b;\
             \nc {d: -E-URL($a)}\n"),
            "c {\
         \n  d: -E-URL(b);\
         \n}\n"
        );
    }
}
