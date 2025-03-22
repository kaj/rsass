//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector")
}

mod complex {
    use super::runner;

    #[test]
    fn adjacent_sibling() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b + c + d\");\
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
    mod bogus {
        use super::runner;

        #[test]
        fn leading() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"> b\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (\">\" b,);\
             \n}\n"),
                "a {\
         \n  result: > b;\
         \n  structure: true;\
         \n}\n"
            );
        }
        mod multiple {
            use super::runner;

            #[test]
            fn middle() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b + ~ c\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \"+\" \"~\" c,);\
             \n}\n"),
                    "a {\
         \n  result: b + ~ c;\
         \n  structure: true;\
         \n}\n"
                );
            }
            #[test]
            fn trailing() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b ~~\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \"~\" \"~\",);\
             \n}\n"),
                    "a {\
         \n  result: b ~ ~;\
         \n  structure: true;\
         \n}\n"
                );
            }
        }
        #[test]
        fn only() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\">\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \">\"),);\
             \n}\n"),
                "a {\
         \n  result: >;\
         \n  structure: true;\
         \n}\n"
            );
        }
        #[test]
        fn trailing() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b +\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (b \"+\",);\
             \n}\n"),
                "a {\
         \n  result: b +;\
         \n  structure: true;\
         \n}\n"
            );
        }
    }
    #[test]
    fn child() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b > c > d\");\
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
            runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b c d\");\
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
            runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b ~ c ~ d\");\
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
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\"b.c:d\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \"b.c:d\"),);\
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
        runner().ok("@use \"sass:selector\";\
             \n$result: selector.parse(\"b c, d e, f g\");\
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
    use super::runner;

    #[test]
    fn attribute() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"[c^=d]\")}\n"),
            "a {\
         \n  b: [c^=d];\
         \n}\n"
        );
    }
    #[test]
    fn class() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\".c\")}\n"),
            "a {\
         \n  b: .c;\
         \n}\n"
        );
    }
    #[test]
    fn id() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"#c\")}\n"),
            "a {\
         \n  b: #c;\
         \n}\n"
        );
    }
    #[test]
    fn placeholder() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"%c\")}\n"),
            "a {\
         \n  b: %c;\
         \n}\n"
        );
    }
    mod pseudo {
        use super::runner;

        mod class {
            use super::runner;

            #[test]
            fn arg() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\":c(@#$)\")}\n"),
                    "a {\
         \n  b: :c(@#$);\
         \n}\n"
                );
            }
            #[test]
            fn combined_arg() {
                assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\":nth-child(2n+1 of b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \":nth-child(2n+1 of b, c)\"),);\
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
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\":c\")}\n"),
                    "a {\
         \n  b: :c;\
         \n}\n"
                );
            }
            mod selector_arg {
                use super::runner;

                #[test]
                fn is() {
                    assert_eq!(
                        runner().ok("@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\":is(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \":is(b, c)\"),);\
             \n}\n"),
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
            "@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\":matches(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \":matches(b, c)\"),);\
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
                        runner().ok("@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\":where(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \":where(b, c)\"),);\
             \n}\n"),
                        "a {\
         \n  result: :where(b, c);\
         \n  structure: true;\
         \n}\n"
                    );
                }
            }
        }
        mod element {
            use super::runner;

            #[test]
            fn arg() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"::c(@#$)\")}\n"),
                    "a {\
         \n  b: ::c(@#$);\
         \n}\n"
                );
            }
            #[test]
            fn no_arg() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"::c\")}\n"),
                    "a {\
         \n  b: ::c;\
         \n}\n"
                );
            }
            #[test]
            fn selector_arg() {
                assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \n$result: selector.parse(\"::slotted(b, c)\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (list.append((), \"::slotted(b, c)\"),);\
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
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"c\")}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn universal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"*\")}\n"),
            "a {\
         \n  b: *;\
         \n}\n"
        );
    }
}
