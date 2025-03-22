//! Tests auto-converted from "sass-spec/spec/libsass/variables_in_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variables_in_media")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$media1: screen;\
             \n$media2: print;\
             \n$var: -webkit-min-device-pixel-ratio;\
             \n$val: 20;\
             \n@media #{$media1} and ($var: $val), only #{$media2} {a {b: c}}\n"
        ),
        "@media screen and (-webkit-min-device-pixel-ratio: 20), only print {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
