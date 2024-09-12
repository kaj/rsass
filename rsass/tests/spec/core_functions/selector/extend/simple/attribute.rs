//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/attribute.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("attribute")
}

#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"[c=d]\", \"[c=d]\", \"e\")}\n"),
        "a {\
         \n  b: [c=d], e;\
         \n}\n"
    );
}
mod unequal {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn name() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"[c=d]\", \"[e=d]\", \"f\")}\n"),
            "a {\
         \n  b: [c=d];\
         \n}\n"
        );
    }
    #[test]
    fn operator() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"[c=d]\", \"[c^=d]\", \"f\")}\n"),
            "a {\
         \n  b: [c=d];\
         \n}\n"
        );
    }
    #[test]
    fn value() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"[c=d]\", \"[c=e]\", \"f\")}\n"),
            "a {\
         \n  b: [c=d];\
         \n}\n"
        );
    }
}
