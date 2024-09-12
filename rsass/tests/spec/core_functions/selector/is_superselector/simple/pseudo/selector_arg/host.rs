//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/host.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("host")
}

#[test]
fn bare_sub() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":host(c d, e f, g h)\", \"c d, e f, g h\")}\n"
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
             \na {b: selector.is-superselector(\":-pfx-host(c d.i, e j f)\", \":-pfx-host(c d, e f, g h)\")}\n"
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
             \na {b: selector.is-superselector(\":-pfx-host(c d, e f, g h)\", \":-pfx-host(c d.i, e j f)\")}\n"
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
             \na {b: selector.is-superselector(\":host(c d.i, e j f)\", \":host(c d, e f, g h)\")}\n"
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
             \na {b: selector.is-superselector(\":host(c d, e f, g h)\", \":host(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
