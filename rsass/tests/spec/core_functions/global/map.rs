//! Tests auto-converted from "sass-spec/spec/core_functions/global/map.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("map")
}

#[test]
fn get() {
    assert_eq!(
        runner().ok("a {b: map-get((c: d), c)}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
#[test]
fn has_key() {
    assert_eq!(
        runner().ok("a {b: map-has-key((c: d), c)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn keys() {
    assert_eq!(
        runner().ok("a {b: map-keys((c: d))}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn merge() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(map-merge((c: d), (e: f)))}\n"),
        "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
    );
}
#[test]
fn remove() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(map-remove((c: d), c))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
#[test]
fn values() {
    assert_eq!(
        runner().ok("a {b: map-values((c: d))}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
