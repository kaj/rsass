//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/universal.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("universal")
}

#[test]
fn and_class() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*\", \".c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn and_type() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*\", \"c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*\", \"*\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod namespace {
    use super::runner;

    mod empty {
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \".d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"c|d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"c|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
    mod explicit {
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \".d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"|d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            mod explicit {
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"c|d\")}\n"),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"e|d\")}\n"),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            mod explicit {
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"c|*\")}\n"),
                        "a {\
         \n  b: true;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"d|*\")}\n"),
                        "a {\
         \n  b: false;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
    }
    mod universal {
        use super::runner;

        #[test]
        fn and_class() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \".d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"c|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"c|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|*\", \"*|*\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
        }
    }
}
