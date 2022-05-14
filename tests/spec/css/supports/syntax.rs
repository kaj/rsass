//! Tests auto-converted from "sass-spec/spec/css/supports/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod anything {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn ident_only() {
        assert_eq!(
            runner().ok("@supports (a) {@b}\n"),
            "@supports (a) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    fn idents() {
        assert_eq!(
            runner().ok("@supports (a b) {@c}\n"),
            "@supports (a b) {\
         \n  @c;\
         \n}\n"
        );
    }
    mod interpolated_any_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn full() {
            assert_eq!(
                runner().ok("@supports (a #{1 + 1}) {@b}\n"),
                "@supports (a 2) {\
         \n  @b;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                runner().ok("@supports (a <#{1 + 1}>) {@b}\n"),
                "@supports (a <2>) {\
         \n  @b;\
         \n}\n"
            );
        }
    }
    mod interpolated_ident {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn full() {
            assert_eq!(
                runner().ok("@supports (#{\"a\"} b) {@c}\n"),
                "@supports (a b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn full_before_andlike() {
            assert_eq!(
                runner().ok("@supports (#{\"a\"} andb) {@c}\n"),
                "@supports (a andb) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn partial() {
            assert_eq!(
                runner().ok("@supports (a#{\"b\"}c d) {@e}\n"),
                "@supports (abc d) {\
         \n  @e;\
         \n}\n"
            );
        }
    }
    #[test]
    fn no_space() {
        assert_eq!(
            runner().ok("@supports (a!) {@b}\n"),
            "@supports (a!) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn only_space() {
        assert_eq!(
            runner().ok("@supports (a ) {@b}\n"),
            "@supports (a ) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn symbols() {
        assert_eq!(
            runner().ok("@supports (a !&$ZH()&;*{&A}_=-+#/><) {@b}\n"),
            "@supports (a !&$ZH()&;*{&A}_=-+#/><) {\
         \n  @b;\
         \n}\n"
        );
    }
}
mod calculations {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn contains_interpolation() {
            assert_eq!(
                runner().ok("@supports (a: calc(#{1 + 2})) {@d}\n"),
                "@supports (a: calc(3)) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn in_property_name() {
            assert_eq!(
                runner().ok("@supports (calc(0): a) {@d}\n"),
                "@supports (calc(0): a) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        fn interpolated() {
            assert_eq!(
                runner().ok("@supports (a: #{calc(1 + 2)}) {@d}\n"),
                "@supports (a: 3) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn nested() {
            assert_eq!(
                runner().ok("@supports (a: calc(1 + calc(2 + 3))) {@d}\n"),
                "@supports (a: calc(1 + calc(2 + 3))) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple() {
            assert_eq!(
                runner().ok("@supports (a: calc(0)) {@d}\n"),
                "@supports (a: calc(0)) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn with_operation() {
            assert_eq!(
                runner().ok("@supports (a: calc(1 + 2)) {@d}\n"),
                "@supports (a: calc(1 + 2)) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn with_variable() {
            assert_eq!(
                runner().ok("$x: 2;\
             \n@supports (a: calc(1 + $x)) {@d}\n"),
                "@supports (a: calc(1 + 2)) {\
         \n  @d;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn clamp() {
        assert_eq!(
            runner().ok("@supports (a: clamp(0, 1, 2)) {@d}\n"),
            "@supports (a: clamp(0, 1, 2)) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@supports (a: max(0)) {@d}\n"),
            "@supports (a: max(0)) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@supports (a: min(0)) {@d}\n"),
            "@supports (a: min(0)) {\
         \n  @d;\
         \n}\n"
        );
    }
}
mod declaration {
    #[allow(unused)]
    use super::runner;

    mod custom_prop {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn comma() {
            assert_eq!(
                runner().ok("@supports (--a: ,) {@c}\n"),
                "@supports (--a: ,) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn value() {
            assert_eq!(
                runner().ok("@supports (--a: b) {@c}\n"),
                "@supports (--a: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn whitespace() {
            assert_eq!(
                runner().ok("@supports (--a: ) {@c}\n"),
                "@supports (--a: ) {\
         \n  @c;\
         \n}\n"
            );
        }
    }
    mod dynamic {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lhs() {
            assert_eq!(
                runner().ok("@supports (1 + 1: b) {@c}\n"),
                "@supports (2: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn rhs() {
            assert_eq!(
                runner().ok("@supports (a: 1 + 1) {@c}\n"),
                "@supports (a: 2) {\
         \n  @c;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn nested() {
        assert_eq!(
            runner().ok("@supports ((((a: b)))) {@c}\n"),
            "@supports (a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    mod plain {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn ident() {
            assert_eq!(
                runner().ok("@supports (a: b) {@c}\n"),
                "@supports (a: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn quoted_rhs() {
            assert_eq!(
                runner().ok("@supports (a: \"b\") {@c}\n"),
                "@supports (a: \"b\") {\
         \n  @c;\
         \n}\n"
            );
        }
    }
}
mod function {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn after_not() {
        assert_eq!(
            runner().ok("@supports not a() {@b}\n"),
            "@supports not a() {\
         \n  @b;\
         \n}\n"
        );
    }
    mod interpolated_name {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn full() {
            assert_eq!(
                runner().ok("@supports #{\"a\"}(b) {@c}\n"),
                "@supports a(b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn partial() {
            assert_eq!(
                runner().ok("@supports a#{\"b\"}c(d) {@e}\n"),
                "@supports abc(d) {\
         \n  @e;\
         \n}\n"
            );
        }
    }
    mod interpolated_value {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn full() {
            assert_eq!(
                runner().ok("@supports a(#{1 + 1}) {@c}\n"),
                "@supports a(2) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                runner().ok("@supports a(<#{1 + 1}>) {@c}\n"),
                "@supports a(<2>) {\
         \n  @c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn no_arg() {
        assert_eq!(
            runner().ok("@supports a() {@b}\n"),
            "@supports a() {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    fn plain() {
        assert_eq!(
            runner().ok("@supports a(b) {@c}\n"),
            "@supports a(b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn space() {
        assert_eq!(
            runner().ok("@supports a( ) {@b}\n"),
            "@supports a( ) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn symbols() {
        assert_eq!(
            runner().ok("@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {@b}\n"),
            "@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {\
         \n  @b;\
         \n}\n"
        );
    }
}
mod lone_interpolation {
    #[allow(unused)]
    use super::runner;

    mod parens {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after_operator() {
            assert_eq!(
                runner()
                    .ok("@supports ((c: 1 + 1) and #{\"(a: b)\"})  {@d}\n"),
                "@supports (c: 2) and (a: b) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        fn alone() {
            assert_eq!(
                runner().ok("@supports (#{\"(a: b)\"}) {@c}\n"),
                "@supports ((a: b)) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before_operator() {
            assert_eq!(
                runner()
                    .ok("@supports (#{\"(a: b)\"} and (c: 1 + 1)) {@d}\n"),
                "@supports (a: b) and (c: 2) {\
         \n  @d;\
         \n}\n"
            );
        }
    }
    mod top_level {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after_operator() {
            assert_eq!(
                runner().ok("@supports (c: 1 + 1) and #{\"(a: b)\"}  {@d}\n"),
                "@supports (c: 2) and (a: b) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        fn alone() {
            assert_eq!(
                runner().ok("@supports #{\"(a: b)\"} {@c}\n"),
                "@supports (a: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        fn before_operator() {
            assert_eq!(
                runner().ok("@supports #{\"(a: b)\"} and (c: 1 + 1) {@d}\n"),
                "@supports (a: b) and (c: 2) {\
         \n  @d;\
         \n}\n"
            );
        }
    }
}
mod operator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and() {
        assert_eq!(
            runner().ok("@supports (a: b) and (c: d) and (e: f) {@g}\n"),
            "@supports (a: b) and (c: d) and (e: f) {\
         \n  @g;\
         \n}\n"
        );
    }
    mod mixed {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_in_not() {
            assert_eq!(
                runner().ok("@supports not ((a: b) and (c: d)) {@e}\n"),
                "@supports not ((a: b) and (c: d)) {\
         \n  @e;\
         \n}\n"
            );
        }
        #[test]
        fn and_in_or() {
            assert_eq!(
                runner().ok("@supports ((a: b) and (c: d)) or (e: f) {@g}\n"),
                "@supports ((a: b) and (c: d)) or (e: f) {\
         \n  @g;\
         \n}\n"
            );
        }
        #[test]
        fn or_in_and() {
            assert_eq!(
                runner().ok("@supports (a: b) and ((c: d) or (e: f)) {@g}\n"),
                "@supports (a: b) and ((c: d) or (e: f)) {\
         \n  @g;\
         \n}\n"
            );
        }
    }
    #[test]
    fn not() {
        assert_eq!(
            runner().ok("@supports not (a: b) {@c}\n"),
            "@supports not (a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn or() {
        assert_eq!(
            runner().ok("@supports (a: b) or (c: d) or (e: f) {@g}\n"),
            "@supports (a: b) or (c: d) or (e: f) {\
         \n  @g;\
         \n}\n"
        );
    }
}
