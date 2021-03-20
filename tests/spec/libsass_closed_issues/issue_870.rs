//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_870.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$quoted-strings-csv: \"alpha\", \"beta\", \'gamma\', \'delta\';\
            \n$quoted-strings-ssv: \"alpha\" \"beta\" \'gamma\' \'delta\';\
            \n\
            \n.csv {\
            \n  output: $quoted-strings-csv;\
            \n  output: #{$quoted-strings-csv};\
            \n  output: \"[#{$quoted-strings-csv}]\";\
            \n  output: \"#{$quoted-strings-csv}\";\
            \n  output: \"[\"#{$quoted-strings-csv}\"]\";\
            \n  output: \'#{$quoted-strings-csv}\';\
            \n  output: \"[\'#{$quoted-strings-csv}\']\";\
            \n}\
            \n\
            \n.ssv {\
            \n  output: $quoted-strings-ssv;\
            \n  output: #{$quoted-strings-ssv};\
            \n  output: \"[#{$quoted-strings-ssv}]\";\
            \n  output: \"#{$quoted-strings-ssv}\";\
            \n  output: \"[\"#{$quoted-strings-ssv}\"]\";\
            \n  output: \'#{$quoted-strings-ssv}\';\
            \n  output: \"[\'#{$quoted-strings-ssv}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".csv {\
        \n  output: \"alpha\", \"beta\", \"gamma\", \"delta\";\
        \n  output: alpha, beta, gamma, delta;\
        \n  output: \"[alpha, beta, gamma, delta]\";\
        \n  output: \"alpha, beta, gamma, delta\";\
        \n  output: \"[\" alpha, beta, gamma, delta \"]\";\
        \n  output: \"alpha, beta, gamma, delta\";\
        \n  output: \"[\'alpha, beta, gamma, delta\']\";\
        \n}\
        \n.ssv {\
        \n  output: \"alpha\" \"beta\" \"gamma\" \"delta\";\
        \n  output: alpha beta gamma delta;\
        \n  output: \"[alpha beta gamma delta]\";\
        \n  output: \"alpha beta gamma delta\";\
        \n  output: \"[\" alpha beta gamma delta \"]\";\
        \n  output: \"alpha beta gamma delta\";\
        \n  output: \"[\'alpha beta gamma delta\']\";\
        \n}\
        \n"
    );
}
