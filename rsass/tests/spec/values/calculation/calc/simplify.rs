//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/simplify.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simplify")
}

mod divide {
    use super::runner;

    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: calc(3px / 2 + 1%)}\n"),
            "a {\
         \n  b: calc(1.5px + 1%);\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 3px / 2)}\n"),
            "a {\
         \n  b: calc(1% + 1.5px);\
         \n}\n"
        );
    }
}
mod invert {
    use super::runner;

    #[test]
    fn minus() {
        assert_eq!(
            runner().ok("a {b: calc(1% - -1px)}\n"),
            "a {\
         \n  b: calc(1% + 1px);\
         \n}\n"
        );
    }
    #[test]
    fn plus() {
        assert_eq!(
            runner().ok("a {b: calc(1% + -1px)}\n"),
            "a {\
         \n  b: calc(1% - 1px);\
         \n}\n"
        );
    }
}
mod minus {
    use super::runner;

    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: calc(3px - 2px + 1%)}\n"),
            "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 3px - 2px)}\n"),
            "a {\
         \n  b: calc(1% + 3px - 2px);\
         \n}\n"
        );
    }
}
#[test]
fn nested() {
    assert_eq!(
        runner().ok("a {b: calc(1% + calc(1px))}\n"),
        "a {\
         \n  b: calc(1% + 1px);\
         \n}\n"
    );
}
mod plus {
    use super::runner;

    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: calc(1px + 2px + 1%)}\n"),
            "a {\
         \n  b: calc(3px + 1%);\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 1px + 2px)}\n"),
            "a {\
         \n  b: calc(1% + 1px + 2px);\
         \n}\n"
        );
    }
}
mod times {
    use super::runner;

    #[test]
    fn left() {
        assert_eq!(
            runner().ok("a {b: calc(3px * 2 + 1%)}\n"),
            "a {\
         \n  b: calc(6px + 1%);\
         \n}\n"
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 3px * 2)}\n"),
            "a {\
         \n  b: calc(1% + 6px);\
         \n}\n"
        );
    }
}
