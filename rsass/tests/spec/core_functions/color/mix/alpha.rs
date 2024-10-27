//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn even() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.3), rgba(#0144bf, 0.3))}\n"),
        "a {\
         \n  b: rgba(73, 146.5, 151, 0.3);\
         \n}\n"
    );
}
#[test]
fn first() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, transparent)}\n"),
        "a {\
         \n  b: rgba(145, 225, 111, 0.5);\
         \n}\n"
    );
}
#[test]
fn firstwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3))}\n"),
        "a {\
         \n  b: rgba(109, 185.75, 131, 0.55);\
         \n}\n"
    );
}
#[test]
fn last() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(transparent, #0144bf)}\n"),
        "a {\
         \n  b: rgba(1, 68, 191, 0.5);\
         \n}\n"
    );
}
#[test]
fn lastwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.4), rgba(#0144bf, 0.9))}\n"),
        "a {\
         \n  b: rgba(37, 107.25, 171, 0.65);\
         \n}\n"
    );
}
