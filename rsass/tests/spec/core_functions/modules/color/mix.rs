//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/mix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mix")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#abcdef, #daddee)}\n"),
        "a {\
         \n  b: rgb(76.2745098039%, 83.5294117647%, 93.5294117647%);\
         \n}\n"
    );
}
