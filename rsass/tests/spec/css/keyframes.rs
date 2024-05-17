//! Tests auto-converted from "sass-spec/spec/css/keyframes.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keyframes")
}

mod bubble {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#611.\
             \na {\
             \n  @keyframes {/**/}\
             \n}\n"),
            "@keyframes { /**/ }\n"
        );
    }
    #[test]
    fn in_mixin() {
        assert_eq!(
            runner().ok("@mixin a {\
             \n  @keyframes b {\
             \n    to { c: d }\
             \n  }\
             \n}\
             \ne {\
             \n  f: g;\
             \n  @include a;\
             \n}\n\n"),
            "e {\
         \n  f: g;\
         \n}\
         \n@keyframes b {\
         \n  to {\
         \n    c: d;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
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
mod error {
    #[allow(unused)]
    use super::runner;

    mod in_keyframe_block {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn style_rule() {
            assert_eq!(
                runner().err(
                    "@keyframes a {\
             \n  to {to {c: d}}\
             \n}\n"
                ),
                "Error: Style rules may not be used within keyframe blocks.\
         \n  ,\
         \n2 |   to {to {c: d}}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod in_keyframe_block {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn known_at_rule() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  to {@media screen {b: c}}\
             \n}\n"),
            "@keyframes a {\
         \n  to {\
         \n    @media screen {\
         \n      b: c;\
         \n    }\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn unknown_at_rule() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  to {@b}\
             \n}\n"),
            "@keyframes a {\
         \n  to {\
         \n    @b;\
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
