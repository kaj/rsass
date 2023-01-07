//! Tests auto-converted from "sass-spec/spec/css/plain/import/conditions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("conditions")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod supports {
        #[allow(unused)]
        use super::runner;

        mod declaration {
            #[allow(unused)]
            use super::runner;

            mod custom_prop {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn empty() {
                    assert_eq!(
                        runner()
                            .err("@import url(\"a.css\") supports(--a:);\n"),
                        "Error: Expected token.\
         \n  ,\
         \n1 | @import url(\"a.css\") supports(--a:);\
         \n  |                                   ^\
         \n  \'\
         \n  input.scss 1:35  root stylesheet",
                    );
                }
            }
        }
    }
    mod wrong_order {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn media_before_supports() {
            assert_eq!(
                runner().err("@import \"a\" (b: c) supports(d: e);\n"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"a\" (b: c) supports(d: e);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn media_before_unknown_function() {
            assert_eq!(
                runner().err("@import \"a\" (b: c) d(e);\n"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"a\" (b: c) d(e);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn media_before_unknown_ident() {
            assert_eq!(
                runner().err("@import \"a\" (b: c) d;\n"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"a\" (b: c) d;\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn supports_after_comma() {
            assert_eq!(
                runner().err("@import \"a\" b, supports(c: d);\n"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"a\" b, supports(c: d);\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn unknown_function_after_comma() {
            assert_eq!(
                runner().err("@import \"a\" b, c(d);\n"),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"a\" b, c(d);\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn url_after_comma() {
            assert_eq!(
                runner().err("@import \"a\" b, \"c\";\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | @import \"a\" b, \"c\";\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
    }
}
mod media {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn complex() {
        assert_eq!(
            runner().ok(
                "@import url(\"a.css\") handheld and (max-width: 400px);\n"
            ),
            "@import url(\"a.css\") handheld and (max-width: 400px);\n"
        );
    }
    mod list {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after_feature() {
            assert_eq!(
                runner().ok("@import \"a\" (b: c), (d: e) and (f: g), h;\n"),
                "@import \"a\" (b: c), (d: e) and (f: g), h;\n"
            );
        }
        #[test]
        fn after_ident() {
            assert_eq!(
                runner().ok("@import \"a\" b, (c: d) and (e: f), g;\n"),
                "@import \"a\" b, (c: d) and (e: f), g;\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn and_without_space() {
            assert_eq!(
                runner().ok("@import \"a\" b and(c: d), e;\n"),
                "@import \"a\" b and (c: d), e;\n"
            );
        }
    }
    #[test]
    fn simple() {
        assert_eq!(
            runner().ok("@import url(\"a.css\") print;\n"),
            "@import url(\"a.css\") print;\n"
        );
    }
}
mod multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn many() {
        assert_eq!(
        runner().ok(
            "@import \"a\" b c d(e) supports(f: g) h i j(k) l m (n: o), (p: q);\n"
        ),
        "@import \"a\" b c d(e) supports(f: g) h i j(k) l m (n: o), (p: q);\n"
    );
    }
    mod supports_then {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn media() {
            assert_eq!(
                runner().ok("@import \"a\" supports(b: c) (d: e);\n"),
                "@import \"a\" supports(b: c) (d: e);\n"
            );
        }
        #[test]
        fn supports() {
            assert_eq!(
                runner().ok("@import \"a\" supports(b: c) supports(e: f);\n"),
                "@import \"a\" supports(b: c) supports(e: f);\n"
            );
        }
        #[test]
        fn unknown_function() {
            assert_eq!(
                runner().ok("@import \"a\" supports(b: c) d(e);\n"),
                "@import \"a\" supports(b: c) d(e);\n"
            );
        }
        #[test]
        fn unknown_ident() {
            assert_eq!(
                runner().ok("@import \"a\" supports(b: c) d;\n"),
                "@import \"a\" supports(b: c) d;\n"
            );
        }
    }
    mod unknown_function_then {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn media() {
            assert_eq!(
                runner().ok("@import \"a\" b(c) (d: e);\n"),
                "@import \"a\" b(c) (d: e);\n"
            );
        }
        #[test]
        fn supports() {
            assert_eq!(
                runner().ok("@import \"a\" b(c) supports(e: f);\n"),
                "@import \"a\" b(c) supports(e: f);\n"
            );
        }
        #[test]
        fn unknown_function() {
            assert_eq!(
                runner().ok("@import \"a\" b(c) d(e);\n"),
                "@import \"a\" b(c) d(e);\n"
            );
        }
        #[test]
        fn unknown_ident() {
            assert_eq!(
                runner().ok("@import \"a\" b(c) d;\n"),
                "@import \"a\" b(c) d;\n"
            );
        }
    }
    mod unknown_ident_then {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn media() {
            assert_eq!(
                runner().ok("@import \"a\" b (c: d);\n"),
                "@import \"a\" b (c: d);\n"
            );
        }
        #[test]
        fn supports() {
            assert_eq!(
                runner().ok("@import \"a\" b supports(c: d);\n"),
                "@import \"a\" b supports(c: d);\n"
            );
        }
        #[test]
        fn unknown_function() {
            assert_eq!(
                runner().ok("@import \"a\" b c(d);\n"),
                "@import \"a\" b c(d);\n"
            );
        }
        #[test]
        fn unknown_ident() {
            assert_eq!(
                runner().ok("@import \"a\" b c;\n"),
                "@import \"a\" b c;\n"
            );
        }
    }
}
mod supports {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn calc() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports(calc(1));\n"),
            "@import \"a.css\" supports(calc(1));\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn condition() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports((a: b));\n"),
            "@import \"a.css\" supports(a: b);\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn condition_and() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports((a: b) and (c: d));\n"),
            "@import \"a.css\" supports((a: b) and (c: d));\n"
        );
    }
    #[test]
    fn condition_function() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports(a(b));\n"),
            "@import \"a.css\" supports(a(b));\n"
        );
    }
    #[test]
    fn condition_negation() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports(not (a: b));\n"),
            "@import \"a.css\" supports(not (a: b));\n"
        );
    }
    mod declaration {
        #[allow(unused)]
        use super::runner;

        mod custom_prop {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn punctuation() {
                assert_eq!(
                    runner().ok("@import \"a.css\" supports(--a: ,);\n"),
                    "@import \"a.css\" supports(--a: ,);\n"
                );
            }
            #[test]
            fn value() {
                assert_eq!(
                    runner().ok("@import \"a.css\" supports(--a: b);\n"),
                    "@import \"a.css\" supports(--a: b);\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn whitespace() {
                assert_eq!(
                    runner().ok("@import \"a.css\" supports(--a: );\n"),
                    "@import \"a.css\" supports(--a: );\n"
                );
            }
        }
        #[test]
        #[ignore] // wrong result
        fn followed_by_import_arg() {
            assert_eq!(
                runner().ok("@import \"a\" supports(b: c), \"d.css\";\n"),
                "@import \"a\" supports(b: c);\
         \n@import \"d.css\";\n"
            );
        }
        #[test]
        fn prop() {
            assert_eq!(
                runner().ok("@import \"a.css\" supports(a: b);\n"),
                "@import \"a.css\" supports(a: b);\n"
            );
        }
    }
}
mod unknown {
    #[allow(unused)]
    use super::runner;

    mod function {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn argument() {
            assert_eq!(
                runner().ok("@import \"a\" b(a !&$ZH()&;*{&A}_=-+#/><);\n"),
                "@import \"a\" b(a !&$ZH()&;*{&A}_=-+#/><);\n"
            );
        }
        #[test]
        fn empty() {
            assert_eq!(
                runner().ok("@import \"a\" b();\n"),
                "@import \"a\" b();\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn followed_by_import_arg() {
            assert_eq!(
                runner().ok("@import \"b\" c(d), \"e.css\";\n"),
                "@import \"b\" c(d);\
         \n@import \"e.css\";\n"
            );
        }
        #[test]
        fn interpolated() {
            assert_eq!(
                runner().ok("$a: \"a\";\
             \n@import \"b\" c(#{$a});\n"),
                "@import \"b\" c(a);\n"
            );
        }
    }
    mod identifier {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn interpolated() {
            assert_eq!(
                runner().ok("$a: \"a\";\
             \n@import \"b\" c#{$a}d;\n"),
                "@import \"b\" cad;\n"
            );
        }
        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok("$a: \"a\";\
             \n@import \"b\" #{$a};\n"),
                "@import \"b\" a;\n"
            );
        }
        #[test]
        fn test_static() {
            assert_eq!(
                runner().ok("@import \"a\" b;\n"),
                "@import \"a\" b;\n"
            );
        }
    }
}
