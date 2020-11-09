//! Tests auto-converted from "sass-spec/spec/css/supports"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/supports/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod syntax {
        #[allow(unused)]
        use super::rsass;
        mod anything {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "colon", error tests are not supported yet.

            // Ignoring "non_identifier_start", error tests are not supported yet.

            // Ignoring "not", error tests are not supported yet.
        }
        mod declaration {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "multiple", error tests are not supported yet.

            // Ignoring "not", error tests are not supported yet.

            // Ignoring "parens", error tests are not supported yet.
        }
        mod function {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "not", error tests are not supported yet.

            // Ignoring "space_before_arg", error tests are not supported yet.
        }
        mod ident {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "interpolated_after", error tests are not supported yet.

            // Ignoring "interpolated_before", error tests are not supported yet.

            // Ignoring "plain", error tests are not supported yet.
        }

        // Ignoring "ident_after_not", error tests are not supported yet.

        // Ignoring "none", error tests are not supported yet.
        mod operator {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "and_after_not", error tests are not supported yet.

            // Ignoring "lonely_not", error tests are not supported yet.

            // Ignoring "not_after_and", error tests are not supported yet.

            // Ignoring "not_function_after_and", error tests are not supported yet.

            // Ignoring "or_after_and", error tests are not supported yet.

            // Ignoring "trailing_and", error tests are not supported yet.

            // Ignoring "trailing_or", error tests are not supported yet.
        }

        // Ignoring "raw_declaration", error tests are not supported yet.
    }
}

// From "sass-spec/spec/css/supports/nesting.hrx"
mod nesting {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            rsass(
                "@supports (a: b) {}\
            \n"
            )
            .unwrap(),
            ""
        );
    }
    #[test]
    #[ignore] // wrong result
    fn invisible() {
        assert_eq!(
            rsass(
                "@supports (a: b) {\
            \n  %c {d: e}\
            \n}\
            \n"
            )
            .unwrap(),
            ""
        );
    }
    #[test]
    fn loud_comment() {
        assert_eq!(
            rsass(
                "// Regression test for sass/libsass#2158\
            \n\
            \n@supports (a: b) {\
            \n  /* c */\
            \n  d {e: f}\
            \n}\
            \n"
            )
            .unwrap(),
            "@supports (a: b) {\
        \n  /* c */\
        \n  d {\
        \n    e: f;\
        \n  }\
        \n}\
        \n"
        );
    }
    mod media {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn in_style_rule() {
            assert_eq!(
                rsass(
                    "c {\
            \n  @media screen {\
            \n    @supports (a: b) {d: e}\
            \n  }\
            \n}\
            \n"
                )
                .unwrap(),
                "@media screen {\
        \n  @supports (a: b) {\
        \n    c {\
        \n      d: e;\
        \n    }\
        \n  }\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn top() {
            assert_eq!(
                rsass(
                    "@media screen {\
            \n  @supports (a: b) {\
            \n    c {d: e}\
            \n  }\
            \n}\
            \n"
                )
                .unwrap(),
                "@media screen {\
        \n  @supports (a: b) {\
        \n    c {\
        \n      d: e;\
        \n    }\
        \n  }\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn style_rule() {
        assert_eq!(
            rsass(
                "a {\
            \n  @supports (b: c) {d: e}\
            \n}\
            \n"
            )
            .unwrap(),
            "@supports (b: c) {\
        \n  a {\
        \n    d: e;\
        \n  }\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn supports() {
        assert_eq!(
            rsass(
                "@supports (a: b) {\
            \n  @supports (c: d) {\
            \n    e {f: g}\
            \n  }\
            \n}\
            \n"
            )
            .unwrap(),
            "@supports (a: b) {\
        \n  @supports (c: d) {\
        \n    e {\
        \n      f: g;\
        \n    }\
        \n  }\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/css/supports/syntax.hrx"
mod syntax {
    #[allow(unused)]
    use super::rsass;
    mod anything {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn ident_only() {
            assert_eq!(
                rsass(
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
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn full() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn full() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod dynamic {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn lhs() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn ident() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn after_not() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn full() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn full() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod parens {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn after_operator() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn after_operator() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn and() {
            assert_eq!(
                rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn and_in_not() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
                rsass(
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
                rsass(
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
}
