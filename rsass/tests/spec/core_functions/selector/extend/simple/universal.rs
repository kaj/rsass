//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/universal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("universal")
}

#[test]
#[ignore] // wrong result
fn and_class() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"*\", \".c\", \"d\")}\n"),
        "a {\
         \n  b: *;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn and_type() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"*\", \"c\", \"d\")}\n"),
        "a {\
         \n  b: *;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn equal() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"*\", \"*\", \"c\")}\n"),
        "a {\
         \n  b: *;\
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
        #[ignore] // wrong result
        fn and_class() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-extend(\"|*\", \".c\", \"d\")}\n"),
                "a {\
         \n  b: |*;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"|*\", \"|d\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn explicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"|*\", \"c|d\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-extend(\"|*\", \"d\", \"e\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"|*\", \"|*\", \"c\")}\n"
                    ),
                    "a {\
         \n  b: |*, c;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn explicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"|*\", \"c|*\", \"d\")}\n"
                    ),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner()
                        .ok("a {b: selector-extend(\"|*\", \"*\", \"c\")}\n"),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn universal() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"|*\", \"*|*\", \"c\")}\n"
                    ),
                    "a {\
         \n  b: |*;\
         \n}\n"
                );
            }
        }
    }
    mod explicit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and_class() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-extend(\"c|*\", \".d\", \"e\")}\n"),
                "a {\
         \n  b: c|*;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"c|*\", \"|d\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            mod explicit {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"c|*\", \"c|d\", \"e\")}\n"
        ),
        "a {\
         \n  b: c|*;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn unequal() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"c|*\", \"e|d\", \"e\")}\n"
        ),
        "a {\
         \n  b: c|*;\
         \n}\n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"c|*\", \"d\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"c|*\", \"|*\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            mod explicit {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn equal() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"c|*\", \"c|*\", \"e\")}\n"
        ),
        "a {\
         \n  b: c|*, e;\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn unequal() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"c|*\", \"d|*\", \"e\")}\n"
        ),
        "a {\
         \n  b: c|*;\
         \n}\n"
    );
                }
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"c|*\", \"*\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn universal() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"c|*\", \"*|*\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: c|*;\
         \n}\n"
                );
            }
        }
    }
    mod universal {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and_class() {
            assert_eq!(
                runner()
                    .ok("a {b: selector-extend(\"*|*\", \".c\", \"d\")}\n"),
                "a {\
         \n  b: *|*;\
         \n}\n"
            );
        }
        mod and_type {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"|c\", \"d\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn explicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"c|d\", \"e\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"c\", \"d\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
        }
        mod and_universal {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn empty() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"|*\", \"c\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn explicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"c|*\", \"d\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn implicit() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"*\", \"c\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn universal() {
                assert_eq!(
                    runner().ok(
                        "a {b: selector-extend(\"*|*\", \"*|*\", \"c\")}\n"
                    ),
                    "a {\
         \n  b: *|*;\
         \n}\n"
                );
            }
        }
    }
}
