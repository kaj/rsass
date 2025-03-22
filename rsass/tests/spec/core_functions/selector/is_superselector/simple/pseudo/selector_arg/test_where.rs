//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/where.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("where")
}

mod both {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c d.i, e j f)\", \":where(c d, e f, g h)\")}\n"
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
             \na {b: selector.is-superselector(\":where(c d, e f, g h)\", \":where(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod complex {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c d e)\", \"c e\")}\n"
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
             \na {b: selector.is-superselector(\":where(c e)\", \"c d e\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod compound {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c.d.e)\", \"c e\")}\n"
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
             \na {b: selector.is-superselector(\":where(c.e)\", \"c.d.e\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod list {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c d, e f)\", \"c d, e f, g h\")}\n"
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
             \na {b: selector.is-superselector(\":where(c d, e f, g h)\", \"c d, e f\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod not_superselector_of {
    use super::runner;

    #[test]
    fn any() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c, d)\", \":any(c, d)\")}\n"
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
             \na {b: selector.is-superselector(\":where(c, d)\", \":-pfx-where(c, d)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
}
mod prefix {
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
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c)\", \"c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":where(c)\", \"d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
