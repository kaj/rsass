//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/33_space_list/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"[\"\'foo   \'\"]\"    \"bar\";\
             \n.result {\
             \n  dquoted: \"#{#{$input}}\";\
             \n  dquoted: \"#{\"[#{$input}]\"}\";\
             \n  dquoted: \"#{\"#{$input}\"}\";\
             \n  dquoted: \"#{\'#{$input}\'}\";\
             \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
             \n  squoted: \'#{#{$input}}\';\
             \n  squoted: \'#{\"[#{$input}]\"}\';\
             \n  squoted: \'#{\"#{$input}\"}\';\
             \n  squoted: \'#{\'#{$input}\'}\';\
             \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
             \n}\n"),
        ".result {\
         \n  dquoted: \"[ foo    ] bar\";\
         \n  dquoted: \"[[ foo    ] bar]\";\
         \n  dquoted: \"[ foo    ] bar\";\
         \n  dquoted: \"[ foo    ] bar\";\
         \n  dquoted: \"[\'[ foo    ] bar\']\";\
         \n  squoted: \"[ foo    ] bar\";\
         \n  squoted: \"[[ foo    ] bar]\";\
         \n  squoted: \"[ foo    ] bar\";\
         \n  squoted: \"[ foo    ] bar\";\
         \n  squoted: \"[\'[ foo    ] bar\']\";\
         \n}\n"
    );
}
