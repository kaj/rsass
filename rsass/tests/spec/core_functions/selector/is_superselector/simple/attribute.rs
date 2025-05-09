//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/attribute.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("attribute")
}

#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"[c=d]\", \"[c=d]\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod unequal {
    use super::runner;

    #[test]
    fn name() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"[c=d]\", \"[e=d]\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn operator() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"[c=d]\", \"[c^=d]\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn value() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"[c=d]\", \"[c=e]\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
