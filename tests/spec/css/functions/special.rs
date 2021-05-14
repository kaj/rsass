//! Tests auto-converted from "sass-spec/spec/css/functions/special.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod clamp {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: clamp(#{0}, #{1}, #{2})}\n"),
            "a {\
         \n  b: clamp(0, 1, 2);\
         \n}\n"
        );
    }
    #[test]
    fn numbers() {
        assert_eq!(
            runner().ok("a {b: clamp(0, 1, 2)}\n"),
            "a {\
         \n  b: clamp(0, 1, 2);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            runner()
                .ok("a {b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"),
            "a {\
         \n  b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\"\"<>,.?/);\
         \n}\n"
        );
    }
}
