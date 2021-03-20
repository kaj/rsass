//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/31_schema_simple/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"[\"\'foo\'\"]\";\
            \n.result {\
            \n  output: $input;\
            \n  output: #{$input};\
            \n  output: \"[#{$input}]\";\
            \n  output: \"#{$input}\";\
            \n  output: \'#{$input}\';\
            \n  output: \"[\'#{$input}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\" \"foo\" \"]\";\
        \n  output: [ foo ];\
        \n  output: \"[[ foo ]]\";\
        \n  output: \"[ foo ]\";\
        \n  output: \"[ foo ]\";\
        \n  output: \"[\'[ foo ]\']\";\
        \n}\
        \n"
    );
}
