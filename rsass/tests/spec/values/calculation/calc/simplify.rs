//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/simplify.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simplify")
}

mod divide {
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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
