//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/32_comma_list/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"[\"\',foo,   \'\"]\";\
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
         \n  dquoted: \"[ ,foo,    ]\";\
         \n  dquoted: \"[[ ,foo,    ]]\";\
         \n  dquoted: \"[ ,foo,    ]\";\
         \n  dquoted: \"[ ,foo,    ]\";\
         \n  dquoted: \"[\'[ ,foo,    ]\']\";\
         \n  squoted: \"[ ,foo,    ]\";\
         \n  squoted: \"[[ ,foo,    ]]\";\
         \n  squoted: \"[ ,foo,    ]\";\
         \n  squoted: \"[ ,foo,    ]\";\
         \n  squoted: \"[\'[ ,foo,    ]\']\";\
         \n}\n"
    );
}
