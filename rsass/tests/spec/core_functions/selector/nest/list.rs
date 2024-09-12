//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("list")
}

mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_final() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d, e\")}\n"),
            "a {\
         \n  b: c d, c e;\
         \n}\n"
        );
    }
    #[test]
    fn initial() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"e\")}\n"),
            "a {\
         \n  b: c e, d e;\
         \n}\n"
        );
    }
    #[test]
    fn many() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"e, f\", \"g, h\")}\n"),
            "a {\
         \n  b: c e g, c e h, c f g, c f h, d e g, d e h, d f g, d f h;\
         \n}\n"
        );
    }
    mod parent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alone() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"&\")}\n"),
                "a {\
         \n  b: c, d;\
         \n}\n"
            );
        }
        #[test]
        fn complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"e &.f\")}\n"),
                "a {\
         \n  b: e c.f, e d.f;\
         \n}\n"
            );
        }
        #[test]
        fn compound() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"&.e\")}\n"),
                "a {\
         \n  b: c.e, d.e;\
         \n}\n"
            );
        }
        #[test]
        fn in_one_complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"&.e, f\")}\n"),
                "a {\
         \n  b: c.e, c f, d.e, d f;\
         \n}\n"
            );
        }
        #[test]
        fn multiple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"&.e &.f\")}\n"),
                "a {\
         \n  b: c.e c.f, c.e d.f, d.e c.f, d.e d.f;\
         \n}\n"
            );
        }
        mod selector_pseudo {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn is() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \":is(&)\")}\n"),
                    "a {\
         \n  b: :is(c, d);\
         \n}\n"
                );
            }
            #[test]
            fn matches() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \":matches(&)\")}\n"),
                    "a {\
         \n  b: :matches(c, d);\
         \n}\n"
                );
            }
            #[test]
            fn test_where() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \":where(&)\")}\n"),
                    "a {\
         \n  b: :where(c, d);\
         \n}\n"
                );
            }
        }
        #[test]
        fn suffix() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c, d\", \"&e\")}\n"),
                "a {\
         \n  b: ce, de;\
         \n}\n"
            );
        }
    }
}
