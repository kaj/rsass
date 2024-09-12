//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/explicit_weight.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("explicit_weight")
}

#[test]
#[ignore] // wrong result
fn even() {
    assert_eq!(
        runner().ok("a {b: mix(#91e16f, #0144bf, 50%)}\n"),
        "a {\
         \n  b: rgb(73, 146.5, 151);\
         \n}\n"
    );
}
#[test]
fn first() {
    assert_eq!(
        runner().ok("a {b: mix(#91e16f, #0144bf, 100%)}\n"),
        "a {\
         \n  b: #91e16f;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn firstwards() {
    assert_eq!(
        runner().ok("a {b: mix(#91e16f, #0144bf, 92%)}\n"),
        "a {\
         \n  b: rgb(133.48, 212.44, 117.4);\
         \n}\n"
    );
}
#[test]
fn last() {
    assert_eq!(
        runner().ok("a {b: mix(#91e16f, #0144bf, 0%)}\n"),
        "a {\
         \n  b: #0144bf;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn lastwards() {
    assert_eq!(
        runner().ok("a {b: mix(#91e16f, #0144bf, 43%)}\n"),
        "a {\
         \n  b: rgb(62.92, 135.51, 156.6);\
         \n}\n"
    );
}
