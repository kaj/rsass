//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/zero-compression.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$orig: 0.12em;\r\
            \n$value: (0.12em);\r\
            \n$score: (item-height: 0.12em);\r\
            \nfoo {\r\
            \n    tst-1: 0 -#{0.12em};\r\
            \n    tst-2: 0 -#{$orig};\r\
            \n    tst-3: 0 -#{$value};\r\
            \n    tst-4: 0 -#{map-get($score, item-height)};\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  tst-1: 0 -0.12em;\
        \n  tst-2: 0 -0.12em;\
        \n  tst-3: 0 -0.12em;\
        \n  tst-4: 0 -0.12em;\
        \n}\
        \n"
    );
}
