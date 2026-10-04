//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/minus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("minus")
}

mod preserved {
    use super::runner;

    #[test]
    fn division() {
        assert_eq!(
            runner().ok("a {b: calc(1px - (2% / var(--c)))}\n"),
            "a {\
         \n  b: calc(1px - 2% / var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn minus() {
        assert_eq!(
            runner().ok("a {b: calc(1px - (2% - var(--c)))}\n"),
            "a {\
         \n  b: calc(1px - (2% - var(--c)));\
         \n}\n"
        );
    }
    #[test]
    fn multiplication() {
        assert_eq!(
            runner().ok("a {b: calc(1px - (2% * var(--c)))}\n"),
            "a {\
         \n  b: calc(1px - 2% * var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: calc(1px - 2%)}\n"),
            "a {\
         \n  b: calc(1px - 2%);\
         \n}\n"
        );
    }
    #[test]
    fn plus() {
        assert_eq!(
            runner().ok("a {b: calc(1px - (2% + var(--c)))}\n"),
            "a {\
         \n  b: calc(1px - (2% + var(--c)));\
         \n}\n"
        );
    }
}
#[test]
fn simplified() {
    assert_eq!(
        runner().ok("a {b: calc(1px - 2px)}\n"),
        "a {\
         \n  b: -1px;\
         \n}\n"
    );
}
