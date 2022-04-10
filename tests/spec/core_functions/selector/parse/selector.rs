//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod complex {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn adjacent_sibling() {
        assert_eq!(
            runner().ok("$result: selector-parse(\"b + c + d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \"+\" c \"+\" d,);\
             \n}\n"),
            "a {\
         \n  result: b + c + d;\
         \n  structure: true;\
         \n}\n"
        );
    }
    #[test]
    fn child() {
        assert_eq!(
            runner().ok("$result: selector-parse(\"b > c > d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \">\" c \">\" d,);\
             \n}\n"),
            "a {\
         \n  result: b > c > d;\
         \n  structure: true;\
         \n}\n"
        );
    }
    #[test]
    fn descendant() {
        assert_eq!(
            runner().ok("$result: selector-parse(\"b c d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b c d,);\
             \n}\n"),
            "a {\
         \n  result: b c d;\
         \n  structure: true;\
         \n}\n"
        );
    }
    #[test]
    fn sibling() {
        assert_eq!(
            runner().ok("$result: selector-parse(\"b ~ c ~ d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \"~\" c \"~\" d,);\
             \n}\n"),
            "a {\
         \n  result: b ~ c ~ d;\
         \n  structure: true;\
         \n}\n"
        );
    }
}
#[test]
fn compound() {
    assert_eq!(
        runner().ok("$result: selector-parse(\"b.c:d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \"b.c:d\"),);\
             \n}\n"),
        "a {\
         \n  result: b.c:d;\
         \n  structure: true;\
         \n}\n"
    );
}
#[test]
fn list() {
    assert_eq!(
        runner().ok("$result: selector-parse(\"b c, d e, f g\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b c, d e, f g);\
             \n}\n"),
        "a {\
         \n  result: b c, d e, f g;\
         \n  structure: true;\
         \n}\n"
    );
}
mod simple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn attribute() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\"[c^=d]\")}\n"),
            "a {\
         \n  b: [c^=d];\
         \n}\n"
        );
    }
    #[test]
    fn class() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\".c\")}\n"),
            "a {\
         \n  b: .c;\
         \n}\n"
        );
    }
    #[test]
    fn id() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\"#c\")}\n"),
            "a {\
         \n  b: #c;\
         \n}\n"
        );
    }
    #[test]
    fn placeholder() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\"%c\")}\n"),
            "a {\
         \n  b: %c;\
         \n}\n"
        );
    }
    mod pseudo {
        #[allow(unused)]
        use super::runner;

        mod class {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn arg() {
                assert_eq!(
                    runner().ok("a {b: selector-parse(\":c(@#$)\")}\n"),
                    "a {\
         \n  b: :c(@#$);\
         \n}\n"
                );
            }
            #[test]
            fn combined_arg() {
                assert_eq!(
        runner().ok(
            "$result: selector-parse(\":nth-child(2n+1 of b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \":nth-child(2n+1 of b, c)\"),);\
             \n}\n"
        ),
        "a {\
         \n  result: :nth-child(2n+1 of b, c);\
         \n  structure: true;\
         \n}\n"
    );
            }
            #[test]
            fn no_arg() {
                assert_eq!(
                    runner().ok("a {b: selector-parse(\":c\")}\n"),
                    "a {\
         \n  b: :c;\
         \n}\n"
                );
            }
            mod selector_arg {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn is() {
                    assert_eq!(
                        runner().ok(
                            "$result: selector-parse(\":is(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \":is(b, c)\"),);\
             \n}\n"
                        ),
                        "a {\
         \n  result: :is(b, c);\
         \n  structure: true;\
         \n}\n"
                    );
                }
                #[test]
                fn matches() {
                    assert_eq!(
                        runner().ok(
                            "$result: selector-parse(\":matches(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \":matches(b, c)\"),);\
             \n}\n"
                        ),
                        "a {\
         \n  result: :matches(b, c);\
         \n  structure: true;\
         \n}\n"
                    );
                }
                #[test]
                fn test_where() {
                    assert_eq!(
                        runner().ok(
                            "$result: selector-parse(\":where(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \":where(b, c)\"),);\
             \n}\n"
                        ),
                        "a {\
         \n  result: :where(b, c);\
         \n  structure: true;\
         \n}\n"
                    );
                }
            }
        }
        mod element {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn arg() {
                assert_eq!(
                    runner().ok("a {b: selector-parse(\"::c(@#$)\")}\n"),
                    "a {\
         \n  b: ::c(@#$);\
         \n}\n"
                );
            }
            #[test]
            fn no_arg() {
                assert_eq!(
                    runner().ok("a {b: selector-parse(\"::c\")}\n"),
                    "a {\
         \n  b: ::c;\
         \n}\n"
                );
            }
            #[test]
            fn selector_arg() {
                assert_eq!(
                    runner().ok(
                        "$result: selector-parse(\"::slotted(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (append((), \"::slotted(b, c)\"),);\
             \n}\n"
                    ),
                    "a {\
         \n  result: ::slotted(b, c);\
         \n  structure: true;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\"c\")}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn universal() {
        assert_eq!(
            runner().ok("a {b: selector-parse(\"*\")}\n"),
            "a {\
         \n  b: *;\
         \n}\n"
        );
    }
}
