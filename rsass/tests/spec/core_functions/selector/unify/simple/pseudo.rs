//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/pseudo.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pseudo")
}

mod arg {
    use super::runner;

    mod class {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c(@#$)\", \":c(*&^)\")}\n"),
                "a {\
         \n  b: :c(@#$):c(*&^);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c(@#$)\", \":c(@#$)\")}\n"),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
    }
    mod element {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"::c(@#$)\", \"::c(*&^)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::c(@#$)\", \"::c(@#$)\")}\n"),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
    }
}
mod host {
    use super::runner;

    mod arg {
        use super::runner;

        mod preserved {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host(.c)\", \":is(.d)\")}\n"),
                    "a {\
         \n  b: :is(.d):host(.c);\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c)\", \":host(.d)\")}\n"),
                    "a {\
         \n  b: :is(.c):host(.d);\
         \n}\n"
                );
            }
        }
        mod removed {
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host(.c)\", \".d\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\".c\", \":host(.d)\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
    }
    mod argless {
        use super::runner;

        mod class {
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host\", \".c\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\".c\", \":host\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
        mod compound {
            use super::runner;

            mod class_and_selector_pseudo {
                use super::runner;

                #[test]
                fn left() {
                    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host\", \".c:is(.d)\"))}\n"
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
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\".c:is(.d)\", \":host\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
                }
            }
            mod host_and_class {
                use super::runner;

                #[test]
                fn left() {
                    assert_eq!(
                        runner().ok(
                            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host\", \":host.c\"))}\n"
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
                            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host.c\", \":host\"))}\n"
                        ),
                        "a {\
         \n  b: null;\
         \n}\n"
                    );
                }
            }
            mod selector_pseudos {
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn left() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host\", \":is(.c):is(.d)\")}\n"),
                        "a {\
         \n  b: :is(.c):host:is(.d);\
         \n}\n"
                    );
                }
                #[test]
                fn right() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c):is(.d)\", \":host\")}\n"),
                        "a {\
         \n  b: :is(.c):is(.d):host;\
         \n}\n"
                    );
                }
            }
        }
        mod host {
            use super::runner;

            #[test]
            fn arg() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host\", \":host(.c)\")}\n"),
                    "a {\
         \n  b: :host:host(.c);\
         \n}\n"
                );
            }
            #[test]
            fn argless() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host\", \":host\")}\n"),
                    "a {\
         \n  b: :host;\
         \n}\n"
                );
            }
        }
        mod host_context {
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host\", \":host-context(.c)\")}\n"),
                    "a {\
         \n  b: :host:host-context(.c);\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host-context(.c)\", \":host\")}\n"),
                    "a {\
         \n  b: :host-context(.c):host;\
         \n}\n"
                );
            }
        }
        mod pseudo {
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host\", \":hover\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":hover\", \":host\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
        mod selector_pseudo {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host\", \":is(.c)\")}\n"),
                    "a {\
         \n  b: :is(.c):host;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c)\", \":host\")}\n"),
                    "a {\
         \n  b: :is(.c):host;\
         \n}\n"
                );
            }
        }
        mod universal {
            use super::runner;

            #[test]
            fn left() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host\", \"*\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*\", \":host\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
    }
}
mod host_context {
    use super::runner;

    mod preserved {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn left() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host-context(.c)\", \":is(.d)\")}\n"),
                "a {\
         \n  b: :is(.d):host-context(.c);\
         \n}\n"
            );
        }
        #[test]
        fn right() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c)\", \":host-context(.d)\")}\n"),
                "a {\
         \n  b: :is(.c):host-context(.d);\
         \n}\n"
            );
        }
    }
    mod removed {
        use super::runner;

        #[test]
        fn left() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\":host-context(.c)\", \".d\"))}\n"
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
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\".c\", \":host-context(.d)\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
    }
}
mod no_arg {
    use super::runner;

    mod class {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c\", \":d\")}\n"),
                "a {\
         \n  b: :c:d;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c\", \":c\")}\n"),
                "a {\
         \n  b: :c;\
         \n}\n"
            );
        }
    }
    mod different_syntax_same_semantics {
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":after\", \"::after\")}\n"),
                "a {\
         \n  b: :after;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":before\", \"::before\")}\n"),
                "a {\
         \n  b: :before;\
         \n}\n"
            );
        }
        #[test]
        fn first_letter() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\":first-letter\", \"::first-letter\")}\n"
        ),
        "a {\
         \n  b: :first-letter;\
         \n}\n"
    );
        }
        #[test]
        fn first_line() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":first-line\", \"::first-line\")}\n"),
                "a {\
         \n  b: :first-line;\
         \n}\n"
            );
        }
    }
    mod element {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"::c\", \"::d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::c\", \"::c\")}\n"),
                "a {\
         \n  b: ::c;\
         \n}\n"
            );
        }
    }
}
mod selector_arg {
    use super::runner;

    mod is {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c)\", \":is(.d)\")}\n"),
                "a {\
         \n  b: :is(.c):is(.d);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":is(.c)\", \":is(.c)\")}\n"),
                "a {\
         \n  b: :is(.c);\
         \n}\n"
            );
        }
    }
    mod matches {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":matches(.c)\", \":matches(.d)\")}\n"),
                "a {\
         \n  b: :matches(.c):matches(.d);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":matches(.c)\", \":matches(.c)\")}\n"),
                "a {\
         \n  b: :matches(.c);\
         \n}\n"
            );
        }
    }
    mod test_where {
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":where(.c)\", \":where(.d)\")}\n"),
                "a {\
         \n  b: :where(.c):where(.d);\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":where(.c)\", \":where(.c)\")}\n"),
                "a {\
         \n  b: :where(.c);\
         \n}\n"
            );
        }
    }
}
