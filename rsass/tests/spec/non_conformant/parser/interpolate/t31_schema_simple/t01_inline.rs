//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/31_schema_simple/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"[\"\'foo\'\"]\";\
             \n  output: #{\"[\"\'foo\'\"]\"};\
             \n  output: \"[#{\"[\"\'foo\'\"]\"}]\";\
             \n  output: \"#{\"[\"\'foo\'\"]\"}\";\
             \n  output: \'#{\"[\"\'foo\'\"]\"}\';\
             \n  output: \"[\'#{\"[\"\'foo\'\"]\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[\" \"foo\" \"]\";\
         \n  output: [ foo ];\
         \n  output: \"[[ foo ]]\";\
         \n  output: \"[ foo ]\";\
         \n  output: \"[ foo ]\";\
         \n  output: \"[\'[ foo ]\']\";\
         \n}\n"
    );
}
