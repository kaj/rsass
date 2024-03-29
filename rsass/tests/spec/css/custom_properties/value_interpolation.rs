//! Tests auto-converted from "sass-spec/spec/css/custom_properties/value_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".value-interpolation {\
             \n  // Interpolation is the only Sass construct that\'s supported in custom\
             \n  // variables.\
             \n  --alone: #{1 + 2};\
             \n  --in-list: a #{1 + 2} c;\
             \n  --in-ident: foo#{1 + 2}bar;\
             \n  --in-string: \"foo#{1 + 2}bar\";\
             \n  --in-uri: uri(foo#{1 + 2}bar);\
             \n}\n"
        ),
        ".value-interpolation {\
         \n  --alone: 3;\
         \n  --in-list: a 3 c;\
         \n  --in-ident: foo3bar;\
         \n  --in-string: \"foo3bar\";\
         \n  --in-uri: uri(foo3bar);\
         \n}\n"
    );
}
