//! Tests auto-converted from "sass-spec/spec/css/supports/syntax.hrx"

mod anything {
    #[test]
    #[ignore] // unexepected error
    fn ident_only() {
        assert_eq!(
            crate::rsass(
                "@supports (a) {@b}\
            \n"
            )
            .unwrap(),
            "@supports (a) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn idents() {
        assert_eq!(
            crate::rsass(
                "@supports (a b) {@c}\
            \n"
            )
            .unwrap(),
            "@supports (a b) {\
        \n  @c;\
        \n}\
        \n"
        );
    }
    mod interpolated_any_value {
        #[test]
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                crate::rsass(
                    "@supports (a #{1 + 1}) {@b}\
            \n"
                )
                .unwrap(),
                "@supports (a 2) {\
        \n  @b;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                crate::rsass(
                    "@supports (a <#{1 + 1}>) {@b}\
            \n"
                )
                .unwrap(),
                "@supports (a <2>) {\
        \n  @b;\
        \n}\
        \n"
            );
        }
    }
    mod interpolated_ident {
        #[test]
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                crate::rsass(
                    "@supports (#{\"a\"} b) {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a b) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn full_before_andlike() {
            assert_eq!(
                crate::rsass(
                    "@supports (#{\"a\"} andb) {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a andb) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                crate::rsass(
                    "@supports (a#{\"b\"}c d) {@e}\
            \n"
                )
                .unwrap(),
                "@supports (abc d) {\
        \n  @e;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn no_space() {
        assert_eq!(
            crate::rsass(
                "@supports (a!) {@b}\
            \n"
            )
            .unwrap(),
            "@supports (a!) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn only_space() {
        assert_eq!(
            crate::rsass(
                "@supports (a ) {@b}\
            \n"
            )
            .unwrap(),
            "@supports (a ) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn symbols() {
        assert_eq!(
            crate::rsass(
                "@supports (a !&$ZH()&;*{&A}_=-+#/><) {@b}\
            \n"
            )
            .unwrap(),
            "@supports (a !&$ZH()&;*{&A}_=-+#/><) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
}
mod declaration {
    mod dynamic {
        #[test]
        #[ignore] // unexepected error
        fn lhs() {
            assert_eq!(
                crate::rsass(
                    "@supports (1 + 1: b) {@c}\
            \n"
                )
                .unwrap(),
                "@supports (2: b) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn rhs() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: 1 + 1) {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a: 2) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn nested() {
        assert_eq!(
            crate::rsass(
                "@supports ((((a: b)))) {@c}\
            \n"
            )
            .unwrap(),
            "@supports (a: b) {\
        \n  @c;\
        \n}\
        \n"
        );
    }
    mod plain {
        #[test]
        #[ignore] // unexepected error
        fn ident() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a: b) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn quoted_rhs() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: \"b\") {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a: \"b\") {\
        \n  @c;\
        \n}\
        \n"
            );
        }
    }
}
mod function {
    #[test]
    #[ignore] // unexepected error
    fn after_not() {
        assert_eq!(
            crate::rsass(
                "@supports not a() {@b}\
            \n"
            )
            .unwrap(),
            "@supports not a() {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    mod interpolated_name {
        #[test]
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                crate::rsass(
                    "@supports #{\"a\"}(b) {@c}\
            \n"
                )
                .unwrap(),
                "@supports a(b) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                crate::rsass(
                    "@supports a#{\"b\"}c(d) {@e}\
            \n"
                )
                .unwrap(),
                "@supports abc(d) {\
        \n  @e;\
        \n}\
        \n"
            );
        }
    }
    mod interpolated_value {
        #[test]
        #[ignore] // unexepected error
        fn full() {
            assert_eq!(
                crate::rsass(
                    "@supports a(#{1 + 1}) {@c}\
            \n"
                )
                .unwrap(),
                "@supports a(2) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn partial() {
            assert_eq!(
                crate::rsass(
                    "@supports a(<#{1 + 1}>) {@c}\
            \n"
                )
                .unwrap(),
                "@supports a(<2>) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn no_arg() {
        assert_eq!(
            crate::rsass(
                "@supports a() {@b}\
            \n"
            )
            .unwrap(),
            "@supports a() {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn plain() {
        assert_eq!(
            crate::rsass(
                "@supports a(b) {@c}\
            \n"
            )
            .unwrap(),
            "@supports a(b) {\
        \n  @c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn space() {
        assert_eq!(
            crate::rsass(
                "@supports a( ) {@b}\
            \n"
            )
            .unwrap(),
            "@supports a( ) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn symbols() {
        assert_eq!(
            crate::rsass(
                "@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {@b}\
            \n"
            )
            .unwrap(),
            "@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {\
        \n  @b;\
        \n}\
        \n"
        );
    }
}
mod lone_interpolation {
    mod parens {
        #[test]
        #[ignore] // unexepected error
        fn after_operator() {
            assert_eq!(
                crate::rsass(
                    "@supports ((c: 1 + 1) and #{\"(a: b)\"})  {@d}\
            \n"
                )
                .unwrap(),
                "@supports (c: 2) and (a: b) {\
        \n  @d;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn alone() {
            assert_eq!(
                crate::rsass(
                    "@supports (#{\"(a: b)\"}) {@c}\
            \n"
                )
                .unwrap(),
                "@supports ((a: b)) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn before_operator() {
            assert_eq!(
                crate::rsass(
                    "@supports (#{\"(a: b)\"} and (c: 1 + 1)) {@d}\
            \n"
                )
                .unwrap(),
                "@supports (a: b) and (c: 2) {\
        \n  @d;\
        \n}\
        \n"
            );
        }
    }
    mod top_level {
        #[test]
        #[ignore] // unexepected error
        fn after_operator() {
            assert_eq!(
                crate::rsass(
                    "@supports (c: 1 + 1) and #{\"(a: b)\"}  {@d}\
            \n"
                )
                .unwrap(),
                "@supports (c: 2) and (a: b) {\
        \n  @d;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn alone() {
            assert_eq!(
                crate::rsass(
                    "@supports #{\"(a: b)\"} {@c}\
            \n"
                )
                .unwrap(),
                "@supports (a: b) {\
        \n  @c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn before_operator() {
            assert_eq!(
                crate::rsass(
                    "@supports #{\"(a: b)\"} and (c: 1 + 1) {@d}\
            \n"
                )
                .unwrap(),
                "@supports (a: b) and (c: 2) {\
        \n  @d;\
        \n}\
        \n"
            );
        }
    }
}
mod operator {
    #[test]
    #[ignore] // unexepected error
    fn and() {
        assert_eq!(
            crate::rsass(
                "@supports (a: b) and (c: d) and (e: f) {@g}\
            \n"
            )
            .unwrap(),
            "@supports (a: b) and (c: d) and (e: f) {\
        \n  @g;\
        \n}\
        \n"
        );
    }
    mod mixed {
        #[test]
        #[ignore] // unexepected error
        fn and_in_not() {
            assert_eq!(
                crate::rsass(
                    "@supports not ((a: b) and (c: d)) {@e}\
            \n"
                )
                .unwrap(),
                "@supports not ((a: b) and (c: d)) {\
        \n  @e;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn and_in_or() {
            assert_eq!(
                crate::rsass(
                    "@supports ((a: b) and (c: d)) or (e: f) {@g}\
            \n"
                )
                .unwrap(),
                "@supports ((a: b) and (c: d)) or (e: f) {\
        \n  @g;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn or_in_and() {
            assert_eq!(
                crate::rsass(
                    "@supports (a: b) and ((c: d) or (e: f)) {@g}\
            \n"
                )
                .unwrap(),
                "@supports (a: b) and ((c: d) or (e: f)) {\
        \n  @g;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn not() {
        assert_eq!(
            crate::rsass(
                "@supports not (a: b) {@c}\
            \n"
            )
            .unwrap(),
            "@supports not (a: b) {\
        \n  @c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn or() {
        assert_eq!(
            crate::rsass(
                "@supports (a: b) or (c: d) or (e: f) {@g}\
            \n"
            )
            .unwrap(),
            "@supports (a: b) or (c: d) or (e: f) {\
        \n  @g;\
        \n}\
        \n"
        );
    }
}
