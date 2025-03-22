//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/universal.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("universal")
}

#[test]
fn and_class() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*\", \".c\", \"d\")}\n"),
        "a {\
         \n  b: *;\
         \n}\n"
    );
}
#[test]
fn and_type() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*\", \"c\", \"d\")}\n"),
        "a {\
         \n  b: *;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*\", \"*\", \"c\")}\n"),
        "a {\
         \n  b: *;\
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
             \na {b: selector.extend(\"|*\", \".c\", \"d\")}\n"),
                "a {\
         \n  b: |*;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"|d\", \"e\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"c|d\", \"e\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"d\", \"e\")}\n"),
                    "a {\
         \n  b: |*;\
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
             \na {b: selector.extend(\"|*\", \"|*\", \"c\")}\n"),
                    "a {\
         \n  b: |*, c;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"c|*\", \"d\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"*\", \"c\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*\", \"*|*\", \"c\")}\n"),
                    "a {\
         \n  b: |*;\
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
             \na {b: selector.extend(\"c|*\", \".d\", \"e\")}\n"),
                "a {\
         \n  b: c|*;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"|d\", \"e\")}\n"),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            mod explicit {
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"c|d\", \"e\")}\n"),
                        "a {\
         \n  b: c|*;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"e|d\", \"e\")}\n"),
                        "a {\
         \n  b: c|*;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"d\", \"e\")}\n"),
                    "a {\
         \n  b: c|*;\
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
             \na {b: selector.extend(\"c|*\", \"|*\", \"e\")}\n"),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            mod explicit {
                use super::runner;

                #[test]
                fn equal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"c|*\", \"e\")}\n"),
                        "a {\
         \n  b: c|*, e;\
         \n}\n"
                    );
                }
                #[test]
                fn unequal() {
                    assert_eq!(
                        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"d|*\", \"e\")}\n"),
                        "a {\
         \n  b: c|*;\
         \n}\n"
                    );
                }
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"*\", \"e\")}\n"),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*\", \"*|*\", \"e\")}\n"),
                    "a {\
         \n  b: c|*;\
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
             \na {b: selector.extend(\"*|*\", \".c\", \"d\")}\n"),
                "a {\
         \n  b: *|*;\
         \n}\n"
            );
        }
        mod and_type {
            use super::runner;

            #[test]
            fn empty() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"|c\", \"d\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"c|d\", \"e\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"c\", \"d\")}\n"),
                    "a {\
         \n  b: *|*;\
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
             \na {b: selector.extend(\"*|*\", \"|*\", \"c\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            fn explicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"c|*\", \"d\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            fn implicit() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"*\", \"c\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            fn universal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|*\", \"*|*\", \"c\")}\n"),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
        }
    }
}
