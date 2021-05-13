//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/31_schema_simple/03_inline_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\"[\"\'foo\'\"]\"}};\
             \n  output: #{\"[#{\"[\"\'foo\'\"]\"}]\"};\
             \n  output: #{\"#{\"[\"\'foo\'\"]\"}\"};\
             \n  output: #{\'#{\"[\"\'foo\'\"]\"}\'};\
             \n  output: #{\"[\'#{\"[\"\'foo\'\"]\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: [ foo ];\
         \n  output: [[ foo ]];\
         \n  output: [ foo ];\
         \n  output: [ foo ];\
         \n  output: [\'[ foo ]\'];\
         \n}\n"
    );
}
