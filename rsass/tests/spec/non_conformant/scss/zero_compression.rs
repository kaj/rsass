//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/zero-compression.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("zero-compression")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\n\
             \n$orig: 0.12em;\
             \n$value: (0.12em);\
             \n$score: (item-height: 0.12em);\
             \nfoo {\
             \n    tst-1: 0 -#{0.12em};\
             \n    tst-2: 0 -#{$orig};\
             \n    tst-3: 0 -#{$value};\
             \n    tst-4: 0 -#{map.get($score, item-height)};\
             \n}"),
        "foo {\
         \n  tst-1: 0 -0.12em;\
         \n  tst-2: 0 -0.12em;\
         \n  tst-3: 0 -0.12em;\
         \n  tst-4: 0 -0.12em;\
         \n}\n"
    );
}
