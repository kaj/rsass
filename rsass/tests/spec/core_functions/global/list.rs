//! Tests auto-converted from "sass-spec/spec/core_functions/global/list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("list")
}

#[test]
fn append() {
    assert_eq!(
        runner().ok("a {b: append(c d, e)}\n"),
        "a {\
         \n  b: c d e;\
         \n}\n"
    );
}
#[test]
fn index() {
    assert_eq!(
        runner().ok("a {b: index(a b c, b)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn is_bracketed() {
    assert_eq!(
        runner().ok("a {b: is-bracketed(a b c)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn join() {
    assert_eq!(
        runner().ok("a {b: join(c d, e f)}\n"),
        "a {\
         \n  b: c d e f;\
         \n}\n"
    );
}
#[test]
fn length() {
    assert_eq!(
        runner().ok("a {b: length(a b c)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn list_separator() {
    assert_eq!(
        runner().ok("a {b: list-separator(a b c)}\n"),
        "a {\
         \n  b: space;\
         \n}\n"
    );
}
#[test]
fn nth() {
    assert_eq!(
        runner().ok("a {b: nth(a b c, 3)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn set_nth() {
    assert_eq!(
        runner().ok("a {b: set-nth(c d e, 2, f)}\n"),
        "a {\
         \n  b: c f e;\
         \n}\n"
    );
}
#[test]
fn zip() {
    assert_eq!(
        runner().ok("a {b: zip(1 2 3, c d e)}\n"),
        "a {\
         \n  b: 1 c, 2 d, 3 e;\
         \n}\n"
    );
}
