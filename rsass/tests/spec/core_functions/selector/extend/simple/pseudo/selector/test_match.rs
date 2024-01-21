//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/match.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("match")
}

mod prefixed {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            runner().ok("a {\
             \n  b: selector-extend(\
             \n      \":nth-child(2n + 1 of c d.e, f g)\",\
             \n      \":nth-child(2n + 1 of c d.e, f g)\",\
             \n      \"h\");\
             \n}\n"),
            "a {\
         \n  b: :nth-child(2n+1 of c d.e, f g), h;\
         \n}\n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn argument() {
            assert_eq!(
                runner().ok("a {\
             \n  b: selector-extend(\
             \n      \":nth-child(2n + 1 of c d.e, f g)\",\
             \n      \":nth-child(2n + 1 of d, g)\",\
             \n      \"h\");\
             \n}\n"),
                "a {\
         \n  b: :nth-child(2n+1 of c d.e, f g);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn has_argument() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":nth-child(2n + 1 of c d.e, f g)\", \":nth-child\", \"h\")}\n"
        ),
        "a {\
         \n  b: :nth-child(2n+1 of c d.e, f g);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn name() {
            assert_eq!(
                runner().ok("a {\
             \n  b: selector-extend(\
             \n      \":nth-child(2n + 1 of c d.e, f g)\",\
             \n      \":nth-last-child(2n + 1 of c d.e, f g)\",\
             \n      \"h\");\
             \n}\n"),
                "a {\
         \n  b: :nth-child(2n+1 of c d.e, f g);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn prefix() {
            assert_eq!(
                runner().ok("a {\
             \n  b: selector-extend(\
             \n      \":nth-child(2n + 1 of c d.e, f g)\",\
             \n      \":nth-child(2n of c d.e, f g)\",\
             \n      \"h\");\
             \n}\n"),
                "a {\
         \n  b: :nth-child(2n+1 of c d.e, f g);\
         \n}\n"
            );
        }
    }
}
mod unprefixed {
    #[allow(unused)]
    use super::runner;

    mod element {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(c.d, e.f)\", \"::slotted(c.d, e.f)\", \"g\")}\n"
        ),
        "a {\
         \n  b: ::slotted(c.d, e.f), g;\
         \n}\n"
    );
        }
        mod unequal {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn argument() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(c.d, e.f)\", \"::slotted(d, g)\", \"g\")}\n"
        ),
        "a {\
         \n  b: ::slotted(c.d, e.f);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn has_argument() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(c.d, e.f)\", \"::slotted\", \"g\")}\n"
        ),
        "a {\
         \n  b: ::slotted(c.d, e.f);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn name() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(c.d, e.f)\", \"::-pfx-slotted(c.d, e.f)\", \"g\")}\n"
        ),
        "a {\
         \n  b: ::slotted(c.d, e.f);\
         \n}\n"
    );
            }
        }
    }
    mod is {
        #[allow(unused)]
        use super::runner;

        mod class {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn equal() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":is(c d.e, f g)\", \":is(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :is(c d.e, f g), h;\
         \n}\n"
    );
            }
            mod unequal {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":is(c d.e, f g)\", \":is(d, g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :is(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn has_argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":is(c d.e, f g)\", \":is\", \"h\")}\n"
        ),
        "a {\
         \n  b: :is(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn name() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":is(c d.e, f g)\", \":-pfx-is(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :is(c d.e, f g);\
         \n}\n"
    );
                }
            }
        }
    }
    mod matches {
        #[allow(unused)]
        use super::runner;

        mod class {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn equal() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":matches(c d.e, f g)\", \":matches(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :matches(c d.e, f g), h;\
         \n}\n"
    );
            }
            mod unequal {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":matches(c d.e, f g)\", \":matches(d, g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :matches(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn has_argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":matches(c d.e, f g)\", \":matches\", \"h\")}\n"
        ),
        "a {\
         \n  b: :matches(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn name() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":matches(c d.e, f g)\", \":-pfx-matches(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :matches(c d.e, f g);\
         \n}\n"
    );
                }
            }
        }
    }
    mod test_where {
        #[allow(unused)]
        use super::runner;

        mod class {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn equal() {
                assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(c d.e, f g)\", \":where(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :where(c d.e, f g), h;\
         \n}\n"
    );
            }
            mod unequal {
                #[allow(unused)]
                use super::runner;

                #[test]
                #[ignore] // wrong result
                fn argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(c d.e, f g)\", \":where(d, g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :where(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn has_argument() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(c d.e, f g)\", \":where\", \"h\")}\n"
        ),
        "a {\
         \n  b: :where(c d.e, f g);\
         \n}\n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn name() {
                    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(c d.e, f g)\", \":-pfx-where(c d.e, f g)\", \"h\")}\n"
        ),
        "a {\
         \n  b: :where(c d.e, f g);\
         \n}\n"
    );
                }
            }
        }
    }
}
