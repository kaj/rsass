//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1127.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: to-upper-case(\'abcd\');\
            \n$b: to-upper-case(\"abcd\");\
            \n$c: to-upper-case(abcd);\
            \n\
            \nfoo {\
            \n    content: #{$a};\
            \n    content: #{$b};\
            \n    content: #{$c};\
            \n    content: \'#{$a}\';\
            \n    content: \'#{$b}\';\
            \n    content: \'#{$c}\';\
            \n    content: \"#{$a}\";\
            \n    content: \"#{$b}\";\
            \n    content: \"#{$c}\";\
            \n\
            \n    content: #{unquote($a)};\
            \n    content: #{unquote($b)};\
            \n    content: #{unquote($c)};\
            \n    content: \'#{unquote($a)}\';\
            \n    content: \'#{unquote($b)}\';\
            \n    content: \'#{unquote($c)}\';\
            \n    content: \"#{unquote($a)}\";\
            \n    content: \"#{unquote($b)}\";\
            \n    content: \"#{unquote($c)}\";\
            \n\
            \n    content: #{$a + unquote(\"efg\")};\
            \n    content: #{$b + unquote(\"efg\")};\
            \n    content: #{$c + unquote(\"efg\")};\
            \n    content: \'#{$a + unquote(\"efg\")}\';\
            \n    content: \'#{$b + unquote(\"efg\")}\';\
            \n    content: \'#{$c + unquote(\"efg\")}\';\
            \n    content: \"#{$a + unquote(\"efg\")}\";\
            \n    content: \"#{$b + unquote(\"efg\")}\";\
            \n    content: \"#{$c + unquote(\"efg\")}\";\
            \n\
            \n    content: #{$a + unquote(\"\")};\
            \n    content: #{$b + unquote(\"\")};\
            \n    content: #{$c + unquote(\"\")};\
            \n    content: \'#{$a + unquote(\"\")}\';\
            \n    content: \'#{$b + unquote(\"\")}\';\
            \n    content: \'#{$c + unquote(\"\")}\';\
            \n    content: \"#{$a + unquote(\"\")}\";\
            \n    content: \"#{$b + unquote(\"\")}\";\
            \n    content: \"#{$c + unquote(\"\")}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: ABCDefg;\
        \n  content: ABCDefg;\
        \n  content: ABCDefg;\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n}\
        \n"
    );
}
