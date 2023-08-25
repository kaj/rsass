//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/current.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("current")
}

#[test]
fn bare_sub() {
    assert_eq!(
        runner().ok(
            "a {b: is-superselector(\":current(c d, e f)\", \"c d, e f\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok(
            "a {b: is-superselector(\":current(c d, e f)\", \":current(c d, e f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod prefix {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
        runner().ok(
            "a {b: is-superselector(\":-pfx-current(c d, e f)\", \":-pfx-current(c d, e f)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    #[test]
    fn subset() {
        assert_eq!(
            runner().ok("a {\
             \n  b: is-superselector(\
             \n      \":-pfx-current(c d.i, e j f)\",\
             \n      \":-pfx-current(c d, e f, g h)\");\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn superset() {
        assert_eq!(
            runner().ok("a {\
             \n  b: is-superselector(\
             \n      \":-pfx-current(c d, e f, g h)\",\
             \n      \":-pfx-current(c d.i, e j f)\");\
             \n}\n"),
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
            "a {b: is-superselector(\":current(c d.i, e j f)\", \":current(c d, e f, g h)\")}\n"
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
            "a {b: is-superselector(\":current(c d, e f, g h)\", \":current(c d.i, e j f)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
