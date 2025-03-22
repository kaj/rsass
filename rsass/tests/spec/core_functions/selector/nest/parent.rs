//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parent")
}

#[test]
fn alone() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"&\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod complex {
    use super::runner;

    #[test]
    fn complex_parent() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c d\", \"e &.f\")}\n"),
            "a {\
         \n  b: e c d.f;\
         \n}\n"
        );
    }
    #[test]
    fn simple_parent() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d &.e\")}\n"),
            "a {\
         \n  b: d c.e;\
         \n}\n"
        );
    }
}
#[test]
fn compound() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"&.d\")}\n"),
        "a {\
         \n  b: c.d;\
         \n}\n"
    );
}
#[test]
fn in_one_complex() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"&.d, e\")}\n"),
        "a {\
         \n  b: c.d, c e;\
         \n}\n"
    );
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"&.d &.e\")}\n"),
        "a {\
         \n  b: c.d c.e;\
         \n}\n"
    );
}
mod selector_pseudo {
    use super::runner;

    mod complex_parent {
        use super::runner;

        #[test]
        fn is() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c d\", \":is(&)\")}\n"),
                "a {\
         \n  b: :is(c d);\
         \n}\n"
            );
        }
        #[test]
        fn matches() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c d\", \":matches(&)\")}\n"),
                "a {\
         \n  b: :matches(c d);\
         \n}\n"
            );
        }
        #[test]
        fn test_where() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c d\", \":where(&)\")}\n"),
                "a {\
         \n  b: :where(c d);\
         \n}\n"
            );
        }
    }
    mod simple_parent {
        use super::runner;

        #[test]
        fn is() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \":is(&)\")}\n"),
                "a {\
         \n  b: :is(c);\
         \n}\n"
            );
        }
        #[test]
        fn matches() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \":matches(&)\")}\n"),
                "a {\
         \n  b: :matches(c);\
         \n}\n"
            );
        }
        #[test]
        fn test_where() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \":where(&)\")}\n"),
                "a {\
         \n  b: :where(c);\
         \n}\n"
            );
        }
    }
}
#[test]
fn suffix() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"&d\")}\n"),
        "a {\
         \n  b: cd;\
         \n}\n"
    );
}
