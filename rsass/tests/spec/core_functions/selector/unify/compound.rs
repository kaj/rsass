//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/compound.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("compound")
}

mod full_overlap {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn class() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.d\", \".c.d\")}\n"),
            "a {\
         \n  b: .c.d;\
         \n}\n"
        );
    }
    #[test]
    fn pseudo_class() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c:d\", \".c:d\")}\n"),
            "a {\
         \n  b: .c:d;\
         \n}\n"
        );
    }
    #[test]
    fn pseudo_element() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c::d\", \".c::d\")}\n"),
            "a {\
         \n  b: .c::d;\
         \n}\n"
        );
    }
    #[test]
    fn pseudo_selector_and_class() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c:d::e\", \".c:d::e\")}\n"),
            "a {\
         \n  b: .c:d::e;\
         \n}\n"
        );
    }
}
#[test]
fn no_overlap() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.d\", \".e.f\")}\n"),
        "a {\
         \n  b: .c.d.e.f;\
         \n}\n"
    );
}
mod order {
    #[allow(unused)]
    use super::runner;

    mod do_not_cross_pseudo_element {
        #[allow(unused)]
        use super::runner;

        mod pseudo_class_and_element {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn into_different_pseudo_element_and_different_pseudo_class() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::foo:bar\", \"::other:baz\")}\n"),
                    ""
                );
            }
            #[test]
            #[ignore] // wrong result
            fn into_pseudo_element() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::bar:baz\", \":foo\")}\n"),
                    "a {\
         \n  b: :foo::bar:baz;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn into_same_pseudo_element_and_different_pseudo_class() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::foo:bar\", \"::foo:baz\")}\n"),
                    "a {\
         \n  b: ::foo:bar:baz;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn into_simple() {
                assert_eq!(
                    runner().ok(
                        "@use \"sass:selector\";\
             \na {b: selector.unify(\".x::scrollbar:horizontal\", \".y\")}\n"
                    ),
                    "a {\
         \n  b: .x.y::scrollbar:horizontal;\
         \n}\n"
                );
            }
        }
        mod pseudo_element {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn into_pseudo_class_and_element() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":foo\", \"::bar:baz\")}\n"),
                    "a {\
         \n  b: :foo::bar:baz;\
         \n}\n"
                );
            }
        }
        mod simple {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn into_pseudo_class_and_element() {
                assert_eq!(
                    runner().ok(
                        "@use \"sass:selector\";\
             \na {b: selector.unify(\".x\", \".y::scrollbar:horizontal\")}\n"
                    ),
                    "a {\
         \n  b: .x.y::scrollbar:horizontal;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn element_at_start() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c\", \"d\")}\n"),
            "a {\
         \n  b: d.c;\
         \n}\n"
        );
    }
    #[test]
    fn preserved_by_default() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.d\", \".e.f\")}\n"),
            "a {\
         \n  b: .c.d.e.f;\
         \n}\n"
        );
    }
    #[test]
    fn pseudo_class_at_end() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c\", \".d\")}\n"),
            "a {\
         \n  b: .d:c;\
         \n}\n"
        );
    }
    mod pseudo_element_after_pseudo_class {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn class_first() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":c\", \"::d\")}\n"),
                "a {\
         \n  b: :c::d;\
         \n}\n"
            );
        }
        #[test]
        fn element_first() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::c\", \":d\")}\n"),
                "a {\
         \n  b: :d::c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn pseudo_element_at_end() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"::c\", \".d\")}\n"),
            "a {\
         \n  b: .d::c;\
         \n}\n"
        );
    }
}
#[test]
fn partial_overlap() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.d\", \".d.e\")}\n"),
        "a {\
         \n  b: .c.d.e;\
         \n}\n"
    );
}
