//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/host_context.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("host_context")
}

#[test]
fn bare_sub() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":host-context(c d, e f, g h)\", \"c d, e f, g h\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod prefix {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":-pfx-host-context(c d.i, e j f)\", \":-pfx-host-context(c d, e f, g h)\")}\n"
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
             \na {b: selector.is-superselector(\":-pfx-host-context(c d, e f, g h)\", \":-pfx-host-context(c d.i, e j f)\")}\n"
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
             \na {b: selector.is-superselector(\":host-context(c d.i, e j f)\", \":host-context(c d, e f, g h)\")}\n"
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
             \na {b: selector.is-superselector(\":host-context(c d, e f, g h)\", \":host-context(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
