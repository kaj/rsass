//! Tests auto-converted from "sass-spec/spec/expressions/if/raw.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("raw")
}

mod attr {
    use super::runner;

    mod adjacent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner().ok("a {b: if(css() attr(--and-clause): c)}\n"),
                "a {\
         \n  b: if(css() attr(--and-clause): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner().ok("a {b: if(attr(--not) css(): c)}\n"),
                "a {\
         \n  b: if(attr(--not) css(): c);\
         \n}\n"
            );
        }
        mod between {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn t1() {
                assert_eq!(
                    runner().ok("a {b: if(css(1) attr(--and) css(2): c)}\n"),
                    "a {\
         \n  b: if(css(1) attr(--and) css(2): c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn t2() {
                assert_eq!(
        runner().ok(
            "a {b: if(css(1) attr(--and) css(2) attr(--and) css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) attr(--and) css(2) attr(--and) css(3): c);\
         \n}\n"
    );
            }
        }
    }
    mod and {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and css(2) attr(--and) css(3): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) attr(--and) css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn and_clause() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and css(2) attr(--and-clause): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) attr(--and-clause): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn clause_and() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and attr(--clause-and) css(2): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and attr(--clause-and) css(2): c);\
         \n}\n"
            );
        }
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn clause_or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or attr(--clause-or) css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) or attr(--clause-or) css(2): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) attr(--or) css(3): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) attr(--or) css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or_clause() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) attr(--or-clause): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) attr(--or-clause): c);\
         \n}\n"
            );
        }
    }
    mod with_sass {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: if((attr(--not) css()) and sass(true): c)}\n"),
                "a {\
         \n  b: if(attr(--not) css(): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and (attr(--not) css()): c)}\n"),
                "a {\
         \n  b: if(attr(--not) css(): c);\
         \n}\n"
            );
        }
    }
}
mod test_if {
    use super::runner;

    mod adjacent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css() if(else: var(--and-clause)): c)}\n"),
                "a {\
         \n  b: if(css() if(else: var(--and-clause)): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner().ok("a {b: if(if(else: var(--not)) css(): c)}\n"),
                "a {\
         \n  b: if(if(else: var(--not)) css(): c);\
         \n}\n"
            );
        }
        mod between {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn t1() {
                assert_eq!(
                    runner().ok(
                        "a {b: if(css(1) if(else: var(--and)) css(2): c)}\n"
                    ),
                    "a {\
         \n  b: if(css(1) if(else: var(--and)) css(2): c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn t2() {
                assert_eq!(
        runner().ok(
            "a {b: if(css(1) if(else: var(--and)) css(2) if(else: var(--and)) css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) if(else: var(--and)) css(2) if(else: var(--and)) css(3): c);\
         \n}\n"
    );
            }
        }
    }
    mod and {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) and css(2) if(else: var(--and)) css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) and css(2) if(else: var(--and)) css(3): c);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn and_clause() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) and css(2) if(else: var(--and-clause)): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) and css(2) if(else: var(--and-clause)): c);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn clause_and() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) and if(else: var(--clause-and)) css(2): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) and if(else: var(--clause-and)) css(2): c);\
         \n}\n"
    );
        }
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn clause_or() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) or if(else: var(--clause-or)) css(2): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) or if(else: var(--clause-or)) css(2): c);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn or() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) or css(2) if(else: var(--or)) css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) or css(2) if(else: var(--or)) css(3): c);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn or_clause() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) or css(2) if(else: var(--or-clause)): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) or css(2) if(else: var(--or-clause)): c);\
         \n}\n"
    );
        }
    }
    mod with_sass {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
        runner().ok(
            "a {b: if((if(else: var(--not)) css()) and sass(true): c)}\n"
        ),
        "a {\
         \n  b: if(if(else: var(--not)) css(): c);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
        runner().ok(
            "a {b: if(sass(true) and (if(else: var(--not)) css()): c)}\n"
        ),
        "a {\
         \n  b: if(if(else: var(--not)) css(): c);\
         \n}\n"
    );
        }
    }
}
mod interp {
    use super::runner;

    mod adjacent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner().ok("a {b: if(css(1) #{\"and css(2)\"}: c)}\n"),
                "a {\
         \n  b: if(css(1) and css(2): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner().ok("a {b: if(#{\"not\"} css(): c)}\n"),
                "a {\
         \n  b: if(not css(): c);\
         \n}\n"
            );
        }
        mod between {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn t1() {
                assert_eq!(
                    runner().ok("a {b: if(css(1) #{\"and\"} css(2): c)}\n"),
                    "a {\
         \n  b: if(css(1) and css(2): c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn t2() {
                assert_eq!(
        runner().ok(
            "a {b: if(css(1) #{\"and\"} css(2) #{\"and\"} css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) and css(2) and css(3): c);\
         \n}\n"
    );
            }
        }
    }
    mod and {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and() {
            assert_eq!(
                runner().ok("a {b: if(css(1) and css(2) and css(3): c)}\n"),
                "a {\
         \n  b: if(css(1) and css(2) and css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn and_clause() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and css(2) #{\"and css(3)\"}: c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) and css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn clause_and() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and #{\"css(2) and\"} css(3): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) and css(3): c);\
         \n}\n"
            );
        }
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn clause_or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or #{\"css(2) or\"} css(3): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) or css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) #{\"or\"} css(3): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) or css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or_clause() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) #{\"or css(3)\"}: c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) or css(3): c);\
         \n}\n"
            );
        }
    }
    mod with_sass {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: if((#{\"not\"} css()) and sass(true): c)}\n"),
                "a {\
         \n  b: if(not css(): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and (#{\"not\"} css()): c)}\n"),
                "a {\
         \n  b: if(not css(): c);\
         \n}\n"
            );
        }
    }
}
mod var {
    use super::runner;

    mod adjacent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner().ok("a {b: if(css() var(--and-clause): c)}\n"),
                "a {\
         \n  b: if(css() var(--and-clause): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner().ok("a {b: if(var(--not) css(): c)}\n"),
                "a {\
         \n  b: if(var(--not) css(): c);\
         \n}\n"
            );
        }
        mod between {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn t1() {
                assert_eq!(
                    runner().ok("a {b: if(css(1) var(--and) css(2): c)}\n"),
                    "a {\
         \n  b: if(css(1) var(--and) css(2): c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn t2() {
                assert_eq!(
        runner().ok(
            "a {b: if(css(1) var(--and) css(2) var(--and) css(3): c)}\n"
        ),
        "a {\
         \n  b: if(css(1) var(--and) css(2) var(--and) css(3): c);\
         \n}\n"
    );
            }
        }
    }
    mod and {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and css(2) var(--and) css(3): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) var(--and) css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn and_clause() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and css(2) var(--and-clause): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and css(2) var(--and-clause): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn clause_and() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) and var(--clause-and) css(2): c)}\n"
                ),
                "a {\
         \n  b: if(css(1) and var(--clause-and) css(2): c);\
         \n}\n"
            );
        }
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn clause_or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or var(--clause-or) css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) or var(--clause-or) css(2): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) var(--or) css(3): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) var(--or) css(3): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or_clause() {
            assert_eq!(
                runner()
                    .ok("a {b: if(css(1) or css(2) var(--or-clause): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2) var(--or-clause): c);\
         \n}\n"
            );
        }
    }
    mod with_sass {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: if((var(--not) css()) and sass(true): c)}\n"),
                "a {\
         \n  b: if(var(--not) css(): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and (var(--not) css()): c)}\n"),
                "a {\
         \n  b: if(var(--not) css(): c);\
         \n}\n"
            );
        }
    }
}
