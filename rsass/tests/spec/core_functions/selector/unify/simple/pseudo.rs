//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/pseudo.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pseudo")
}

mod arg {
    #[allow(unused)]
    use super::runner;

    mod class {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":c(@#$)\", \":c(*&^)\")}\n"),
                "a {\
         \n  b: :c(@#$):c(*&^);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":c(@#$)\", \":c(@#$)\")}\n"),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
    }
    mod element {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\"::c(@#$)\", \"::c(*&^)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok(
                    "a {b: selector-unify(\"::c(@#$)\", \"::c(@#$)\")}\n"
                ),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
    }
}
mod host {
    #[allow(unused)]
    use super::runner;

    mod arg {
        #[allow(unused)]
        use super::runner;

        mod preserved {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn left() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-unify(\":host(.c)\", \":is(.d)\")}\n"
                    ),
                    "a {\
         \n  b: :is(.d):host(.c);\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-unify(\":is(.c)\", \":host(.d)\")}\n"
                    ),
                    "a {\
         \n  b: :is(.c):host(.d);\
         \n}\n"
                );
            }
        }
        mod removed {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host(.c)\", \".d\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
            }
            #[test]
            fn right() {
                assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\".c\", \":host(.d)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
            }
        }
    }
    mod argless {
        #[allow(unused)]
        use super::runner;

        mod class {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok(
                        "a {b: inspect(selector-unify(\":host\", \".c\"))}\n"
                    ),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok(
                        "a {b: inspect(selector-unify(\".c\", \":host\"))}\n"
                    ),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
        mod compound {
            #[allow(unused)]
            use super::runner;

            mod class_and_selector_pseudo {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn left() {
                    assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host\", \".c:is(.d)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
                }
                #[test]
                fn right() {
                    assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\".c:is(.d)\", \":host\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
                }
            }
            mod host_and_class {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn left() {
                    assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host\", \":host.c\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
                }
                #[test]
                fn right() {
                    assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host.c\", \":host\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
                }
            }
            mod selector_pseudos {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn left() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":host\", \":is(.c):is(.d)\")}\n"
        ),
        "a {\
         \n  b: :is(.c):host:is(.d);\
         \n}\n"
    );
                }
                #[test]
                fn right() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":is(.c):is(.d)\", \":host\")}\n"
        ),
        "a {\
         \n  b: :is(.c):is(.d):host;\
         \n}\n"
    );
                }
            }
        }
        mod host {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn arg() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-unify(\":host\", \":host(.c)\")}\n"
                    ),
                    "a {\
         \n  b: :host:host(.c);\
         \n}\n"
                );
            }
            #[test]
            fn argless() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-unify(\":host\", \":host\")}\n"),
                    "a {\
         \n  b: :host;\
         \n}\n"
                );
            }
        }
        mod host_context {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":host\", \":host-context(.c)\")}\n"
        ),
        "a {\
         \n  b: :host:host-context(.c);\
         \n}\n"
    );
            }
            #[test]
            fn right() {
                assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":host-context(.c)\", \":host\")}\n"
        ),
        "a {\
         \n  b: :host-context(.c):host;\
         \n}\n"
    );
            }
        }
        mod pseudo {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host\", \":hover\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
            }
            #[test]
            fn right() {
                assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":hover\", \":host\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
            }
        }
        mod selector_pseudo {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn left() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-unify(\":host\", \":is(.c)\")}\n"
                    ),
                    "a {\
         \n  b: :is(.c):host;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-unify(\":is(.c)\", \":host\")}\n"
                    ),
                    "a {\
         \n  b: :is(.c):host;\
         \n}\n"
                );
            }
        }
        mod universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok(
                        "a {b: inspect(selector-unify(\":host\", \"*\"))}\n"
                    ),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok(
                        "a {b: inspect(selector-unify(\"*\", \":host\"))}\n"
                    ),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
    }
}
mod host_context {
    #[allow(unused)]
    use super::runner;

    mod preserved {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn left() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":host-context(.c)\", \":is(.d)\")}\n"
        ),
        "a {\
         \n  b: :is(.d):host-context(.c);\
         \n}\n"
    );
        }
        #[test]
        fn right() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":is(.c)\", \":host-context(.d)\")}\n"
        ),
        "a {\
         \n  b: :is(.c):host-context(.d);\
         \n}\n"
    );
        }
    }
    mod removed {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn left() {
            assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\":host-context(.c)\", \".d\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
        #[test]
        fn right() {
            assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\".c\", \":host-context(.d)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
    }
}
mod no_arg {
    #[allow(unused)]
    use super::runner;

    mod class {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("a {b: selector-unify(\":c\", \":d\")}\n"),
                "a {\
         \n  b: :c:d;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("a {b: selector-unify(\":c\", \":c\")}\n"),
                "a {\
         \n  b: :c;\
         \n}\n"
            );
        }
    }
    mod different_syntax_same_semantics {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":after\", \"::after\")}\n"),
                "a {\
         \n  b: :after;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":before\", \"::before\")}\n"),
                "a {\
         \n  b: :before;\
         \n}\n"
            );
        }
        #[test]
        fn first_letter() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":first-letter\", \"::first-letter\")}\n"
        ),
        "a {\
         \n  b: :first-letter;\
         \n}\n"
    );
        }
        #[test]
        fn first_line() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":first-line\", \"::first-line\")}\n"
        ),
        "a {\
         \n  b: :first-line;\
         \n}\n"
    );
        }
    }
    mod element {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner()
                    .ok("a {b: inspect(selector-unify(\"::c\", \"::d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("a {b: selector-unify(\"::c\", \"::c\")}\n"),
                "a {\
         \n  b: ::c;\
         \n}\n"
            );
        }
    }
}
mod selector_arg {
    #[allow(unused)]
    use super::runner;

    mod is {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":is(.c)\", \":is(.d)\")}\n"),
                "a {\
         \n  b: :is(.c):is(.d);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-unify(\":is(.c)\", \":is(.c)\")}\n"),
                "a {\
         \n  b: :is(.c);\
         \n}\n"
            );
        }
    }
    mod matches {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":matches(.c)\", \":matches(.d)\")}\n"
        ),
        "a {\
         \n  b: :matches(.c):matches(.d);\
         \n}\n"
    );
        }
        #[test]
        fn same() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\":matches(.c)\", \":matches(.c)\")}\n"
        ),
        "a {\
         \n  b: :matches(.c);\
         \n}\n"
    );
        }
    }
    mod test_where {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok(
                    "a {b: selector-unify(\":where(.c)\", \":where(.d)\")}\n"
                ),
                "a {\
         \n  b: :where(.c):where(.d);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok(
                    "a {b: selector-unify(\":where(.c)\", \":where(.c)\")}\n"
                ),
                "a {\
         \n  b: :where(.c);\
         \n}\n"
            );
        }
    }
}
