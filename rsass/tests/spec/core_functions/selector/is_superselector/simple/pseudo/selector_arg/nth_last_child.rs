//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/nth_last_child.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nth_last_child")
}

#[test]
fn bare_sub() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":nth-last-child(n+1 of c d, e f, g h)\", \"c d, e f, g h\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn bare_super() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \":nth-last-child(n+1 of c)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn different_arg() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.is-superselector(\
             \n      \":nth-last-child(n+1 of c)\",\
             \n      \":nth-last-child(n+2 of c)\");\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod prefix {
    use super::runner;

    #[test]
    fn subset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.is-superselector(\
             \n      \":-pfx-nth-last-child(n+1 of c d.i, e j f)\",\
             \n      \":-pfx-nth-last-child(n+1 of c d, e f, g h)\");\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn superset() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.is-superselector(\
             \n      \":-pfx-nth-last-child(n+1 of c d, e f, g h)\",\
             \n      \":-pfx-nth-last-child(n+1 of c d.i, e j f)\");\
             \n}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
#[test]
fn subset() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.is-superselector(\
             \n      \":nth-last-child(n+1 of c d.i, e j f)\",\
             \n      \":nth-last-child(n+1 of c d, e f, g h)\");\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn superset() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.is-superselector(\
             \n      \":nth-last-child(n+1 of c d, e f, g h)\",\
             \n      \":nth-last-child(n+1 of c d.i, e j f)\");\
             \n}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
