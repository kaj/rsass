//! Tests auto-converted from "sass-spec/spec/css/keyframes.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keyframes")
}

mod bubble {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#611.\
             \na {\
             \n  @keyframes {/**/}\
             \n}\n"),
            "@keyframes {\
         \n  /**/\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn rules() {
        assert_eq!(
            runner().ok("// Regression test for sass/libsass#472\
             \na {\
             \n  b: c;\
             \n  @keyframes d {\
             \n    to {\
             \n      e: f;\
             \n    }\
             \n  }\
             \n}\n"),
            "a {\
         \n  b: c;\
         \n}\
         \n@keyframes d {\
         \n  to {\
         \n    e: f;\
         \n  }\
         \n}\n"
        );
    }
}
mod name {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn interpolated() {
        assert_eq!(
            runner().ok("$a: b;\
             \n@keyframes #{$a} {\
             \n  to {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes b {\
         \n  to {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn variable_like() {
        assert_eq!(
            runner().ok("$a: b;\
             \n@keyframes $a {\
             \n  to {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes $a {\
         \n  to {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
}
mod selector {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn from() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  from {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes a {\
         \n  from {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn interpolated() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  $b: 10%;\
             \n  #{$b} {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes a {\
         \n  10% {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn list() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  from, 15%, to {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes a {\
         \n  from, 15%, to {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    mod percentage {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn double() {
            assert_eq!(
                runner().ok("@keyframes a {\
             \n  10.3% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                "@keyframes a {\
         \n  10.3% {\
         \n    c: d;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn int() {
            assert_eq!(
                runner().ok("@keyframes a {\
             \n  10% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                "@keyframes a {\
         \n  10% {\
         \n    c: d;\
         \n  }\
         \n}\n"
            );
        }
        mod scientific {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn double() {
                assert_eq!(
                    runner().ok("@keyframes a {\
             \n  1.5e2% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                    "@keyframes a {\
         \n  1.5e2% {\
         \n    c: d;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn int() {
                assert_eq!(
                    runner().ok("@keyframes a {\
             \n  1e2% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                    "@keyframes a {\
         \n  1e2% {\
         \n    c: d;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn negative_exponent() {
                assert_eq!(
                    runner().ok("@keyframes a {\
             \n  130E-1% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                    "@keyframes a {\
         \n  130e-1% {\
         \n    c: d;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn positive_exponent() {
                assert_eq!(
                    runner().ok("@keyframes a {\
             \n  13E+1% {\
             \n    c: d;\
             \n  }\
             \n}\n"),
                    "@keyframes a {\
         \n  13e+1% {\
         \n    c: d;\
         \n  }\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn to() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  to {\
             \n    c: d;\
             \n  }\
             \n}\n"),
            "@keyframes a {\
         \n  to {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
}
