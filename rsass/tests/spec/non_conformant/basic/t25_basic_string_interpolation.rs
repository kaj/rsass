//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/25_basic_string_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("25_basic_string_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \ndiv {\
             \n  blah: \"hello #{2+2} world #{math.unit(23px)} #{\'bloo\\n\'} blah\";\
             \n}"
        ),
        "div {\
         \n  blah: \"hello 4 world px bloon blah\";\
         \n}\n"
    );
}
