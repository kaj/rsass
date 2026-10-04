//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/explicit_weight.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("explicit_weight")
}

#[test]
#[ignore] // wrong result
fn even() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 50%)}\n"),
        "a {\
         \n  b: rgb(28.6274509804%, 57.4509803922%, 59.2156862745%);\
         \n}\n"
    );
}
#[test]
fn first() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 100%)}\n"),
        "a {\
         \n  b: #91e16f;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn firstwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 92%)}\n"),
        "a {\
         \n  b: rgb(52.3450980392%, 83.3098039216%, 46.0392156863%);\
         \n}\n"
    );
}
#[test]
fn last() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 0%)}\n"),
        "a {\
         \n  b: #0144bf;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn lastwards() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 43%)}\n"),
        "a {\
         \n  b: rgb(24.6745098039%, 53.1411764706%, 61.4117647059%);\
         \n}\n"
    );
}
