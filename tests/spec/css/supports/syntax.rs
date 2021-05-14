//! Tests auto-converted from "sass-spec/spec/css/supports/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod anything {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn ident_only() {
        assert_eq!(
            runner().ok("@supports (a) {@b}\n"),
            "@supports (a) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
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
        #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                runner().ok("@supports (#{\"a\"} b) {@c}\n"),
                "@supports (a b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn full_before_andlike() {
            assert_eq!(
                runner().ok("@supports (#{\"a\"} andb) {@c}\n"),
                "@supports (a andb) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
mod declaration {
    #[allow(unused)]
    use super::runner;

    mod dynamic {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn lhs() {
            assert_eq!(
                runner().ok("@supports (1 + 1: b) {@c}\n"),
                "@supports (2: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn ident() {
            assert_eq!(
                runner().ok("@supports (a: b) {@c}\n"),
                "@supports (a: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                runner().ok("@supports #{\"a\"}(b) {@c}\n"),
                "@supports a(b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
    fn no_arg() {
        assert_eq!(
            runner().ok("@supports a() {@b}\n"),
            "@supports a() {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn plain() {
        assert_eq!(
            runner().ok("@supports a(b) {@c}\n"),
            "@supports a(b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
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
        #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn alone() {
            assert_eq!(
                runner().ok("@supports (#{\"(a: b)\"}) {@c}\n"),
                "@supports ((a: b)) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn after_operator() {
            assert_eq!(
                runner().ok("@supports (c: 1 + 1) and #{\"(a: b)\"}  {@d}\n"),
                "@supports (c: 2) and (a: b) {\
         \n  @d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn alone() {
            assert_eq!(
                runner().ok("@supports #{\"(a: b)\"} {@c}\n"),
                "@supports (a: b) {\
         \n  @c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
        #[ignore] // unexepected error
        fn and_in_not() {
            assert_eq!(
                runner().ok("@supports not ((a: b) and (c: d)) {@e}\n"),
                "@supports not ((a: b) and (c: d)) {\
         \n  @e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn and_in_or() {
            assert_eq!(
                runner().ok("@supports ((a: b) and (c: d)) or (e: f) {@g}\n"),
                "@supports ((a: b) and (c: d)) or (e: f) {\
         \n  @g;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // unexepected error
    fn not() {
        assert_eq!(
            runner().ok("@supports not (a: b) {@c}\n"),
            "@supports not (a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn or() {
        assert_eq!(
            runner().ok("@supports (a: b) or (c: d) or (e: f) {@g}\n"),
            "@supports (a: b) or (c: d) or (e: f) {\
         \n  @g;\
         \n}\n"
        );
    }
}
