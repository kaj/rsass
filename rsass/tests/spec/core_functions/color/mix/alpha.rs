//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
#[ignore] // wrong result
fn even() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.3), rgba(#0144bf, 0.3))}\n"),
        "a {\
         \n  b: rgba(28.6274509804%, 57.4509803922%, 59.2156862745%, 0.3);\
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
#[ignore] // wrong result
fn firstwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3))}\n"),
        "a {\
         \n  b: rgba(42.7450980392%, 72.8431372549%, 51.3725490196%, 0.55);\
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
#[ignore] // wrong result
fn lastwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.4), rgba(#0144bf, 0.9))}\n"),
        "a {\
         \n  b: rgba(14.5098039216%, 42.0588235294%, 67.0588235294%, 0.65);\
         \n}\n"
    );
}
