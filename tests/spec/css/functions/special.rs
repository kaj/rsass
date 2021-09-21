//! Tests auto-converted from "sass-spec/spec/css/functions/special.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod prefixed {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
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
         \n  b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\"\"<>,.?/);\
         \n}\n"
            );
        }
    }
}
