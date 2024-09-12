//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/any.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("any")
}

mod prefix {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-any(c d.i, e j f)\", \"c d, e f, g h\")}\n"
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
             \na {b: selector.is-superselector(\":-pfx-any(c d, e f, g h)\", \"c d.i, e j f\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
#[test]
fn subset() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":any(c d.i, e j f)\", \"c d, e f, g h\")}\n"
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
             \na {b: selector.is-superselector(\":any(c d, e f, g h)\", \"c d.i, e j f\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
