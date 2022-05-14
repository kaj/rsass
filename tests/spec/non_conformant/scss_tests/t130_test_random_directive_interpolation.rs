//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/130_test_random_directive_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("130_test_random_directive_interpolation")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$domain: \"sass-lang.com\";\
             \n@foo url(https://#{$domain}/),\
             \n     #{domain($domain)},\
             \n     \"foo#{\'ba\' + \'r\'}baz\",\
             \n     foo#{\'ba\' + \'r\'}baz {\
             \n  .foo {a: b}\
             \n}\n"),
        "@foo url(https://sass-lang.com/),\
         \n     domain(\"sass-lang.com\"),\
         \n     \"foobarbaz\",\
         \n     foobarbaz {\
         \n  .foo {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
