//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/unweighted.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unweighted")
}

#[test]
#[ignore] // wrong result
fn average() {
    assert_eq!(
        runner()
            .ok("// All channels should be averaged across the two colors.\
             \na {b: mix(#91e16f, #0144bf)}\n"),
        "a {\
         \n  b: rgb(73, 146.5, 151);\
         \n}\n"
    );
}
#[test]
fn identical() {
    assert_eq!(
        runner().ok(
            "// If two channels have the same values, they should be the same in the output.\
             \na {b: mix(#123456, #123456)}\n"
        ),
        "a {\
         \n  b: #123456;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn min_and_max() {
    assert_eq!(
        runner().ok(
            "// Each channel becomes the average of 255 and 0, which is 128 = 0xAA.\
             \na {b: mix(#ff00ff, #00ff00)}\n"
        ),
        "a {\
         \n  b: rgb(127.5, 127.5, 127.5);\
         \n}\n"
    );
}
