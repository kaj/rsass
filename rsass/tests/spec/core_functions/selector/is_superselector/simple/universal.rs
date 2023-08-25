//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/universal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("universal")
}

#[test]
fn and_class() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"*\", \".c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn and_type() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"*\", \"c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"*\", \"*\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod namespace {
    #[allow(unused)]
    use super::runner;

    mod empty {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"|*\", \".d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"c|d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"c|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
    mod explicit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"c|*\", \".d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"c|*\", \"|d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            mod explicit {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok(
                            "a {b: is-superselector(\"c|*\", \"c|d\")}\n"
                        ),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok(
                            "a {b: is-superselector(\"c|*\", \"e|d\")}\n"
                        ),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"c|*\", \"d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"c|*\", \"|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            mod explicit {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok(
                            "a {b: is-superselector(\"c|*\", \"c|*\")}\n"
                        ),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok(
                            "a {b: is-superselector(\"c|*\", \"d|*\")}\n"
                        ),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"c|*\", \"*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner()
                        .ok("a {b: is-superselector(\"c|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
    mod universal {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"*|*\", \".d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"*|*\", \"|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner()
                        .ok("a {b: is-superselector(\"*|*\", \"c|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"*|*\", \"d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"*|*\", \"|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner()
                        .ok("a {b: is-superselector(\"*|*\", \"c|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("a {b: is-superselector(\"*|*\", \"*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner()
                        .ok("a {b: is-superselector(\"*|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
        }
    }
}
