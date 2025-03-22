//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

#[test]
fn bare_sub() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d, e f, g h)\", \"c d, e f, g h\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod equivalence {
    use super::runner;

    mod split_sub {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d.i, e j f)\", \":not(c d):not(e f):not(g h)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        fn superset() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d, e f, g h)\", \":not(c d.i):not(e j f)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
    }
    mod split_super {
        use super::runner;

        #[test]
        fn subset() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d.i):not(e j f)\", \":not(c d, e f, g h)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
        }
        #[test]
        fn superset() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d):not(e f):not(g h)\", \":not(c d.i, e j f)\")}"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
    }
}
#[test]
#[ignore] // wrong result
fn id() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(#c.d)\", \"#e\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod prefix {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-not(c d.i, e j f)\", \":-pfx-not(c d, e f, g h)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    #[test]
    fn superset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-not(c d, e f, g h)\", \":-pfx-not(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
}
#[test]
fn subset() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d.i, e j f)\", \":not(c d, e f, g h)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn superset() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c d, e f, g h)\", \":not(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn test_type() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":not(c.d)\", \"e\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
