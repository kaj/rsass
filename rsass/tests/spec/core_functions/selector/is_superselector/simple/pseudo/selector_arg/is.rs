//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/is.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is")
}

mod both {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c d.i, e j f)\", \":is(c d, e f, g h)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn superset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c d, e f, g h)\", \":is(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod complex {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c d e)\", \"c e\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c e)\", \"c d e\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod compound {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c.d.e)\", \"c e\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c.e)\", \"c.d.e\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c d, e f)\", \"c d, e f, g h\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c d, e f, g h)\", \"c d, e f\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod not_superselector_of {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn any() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c, d)\", \":any(c, d)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn prefixed() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c, d)\", \":-pfx-is(c, d)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
}
mod prefix {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-is(c d.i, e j f)\", \"c d, e f, g h\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-is(c d, e f, g h)\", \"c d.i, e j f\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod simple {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c)\", \"c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":is(c)\", \"d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
