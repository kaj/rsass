//! Tests auto-converted from "sass-spec/spec/core_functions/global/string.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("string")
}

#[test]
fn index() {
    assert_eq!(
        runner().ok("a {b: str-index(\"c\", \"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn insert() {
    assert_eq!(
        runner().ok("a {b: str-insert(\"c\", \"d\", 1)}\n"),
        "a {\
         \n  b: \"dc\";\
         \n}\n"
    );
}
#[test]
fn length() {
    assert_eq!(
        runner().ok("a {b: str-length(\"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn quote() {
    assert_eq!(
        runner().ok("a {b: quote(c)}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn slice() {
    assert_eq!(
        runner().ok("a {b: str-slice(\"c\", 1, 1)}\n"),
        "a {\
         \n  b: \"c\";\
         \n}\n"
    );
}
#[test]
fn to_upper_case() {
    assert_eq!(
        runner().ok("a {b: to-upper-case(\"c\")}\n"),
        "a {\
         \n  b: \"C\";\
         \n}\n"
    );
}
#[test]
fn unique_id() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(unique-id())}\n"),
        "a {\
         \n  b: string;\
         \n}\n"
    );
}
#[test]
fn unquote() {
    assert_eq!(
        runner().ok("a {b: unquote(\"c\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
