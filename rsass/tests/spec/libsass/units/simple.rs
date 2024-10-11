//! Tests auto-converted from "sass-spec/spec/libsass/units/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \ndiv {\
             \n  hey: ((5in + 3cm) * 10px * 100pt * 10fu / 2px / 2fu / 3pt);\
             \n  ho: (23in/2fu) > (23cm/2fu);\
             \n  hoo: math.unit((23px/2fu/12emu/1.2gnu));\
             \n  hee: math.unit((2in/3cm/4cm));\
             \n}"),
        "div {\
         \n  hey: 370866.1417322835pt;\
         \n  ho: true;\
         \n  hoo: \"px/(fu*emu*gnu)\";\
         \n  hee: \"cm^-1\";\
         \n}\n"
    );
}
