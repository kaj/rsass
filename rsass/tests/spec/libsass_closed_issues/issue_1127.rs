//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1127.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1127")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n$a: string.to-upper-case(\'abcd\');\
             \n$b: string.to-upper-case(\"abcd\");\
             \n$c: string.to-upper-case(abcd);\n\
             \nfoo {\
             \n    content: #{$a};\
             \n    content: #{$b};\
             \n    content: #{$c};\
             \n    content: \'#{$a}\';\
             \n    content: \'#{$b}\';\
             \n    content: \'#{$c}\';\
             \n    content: \"#{$a}\";\
             \n    content: \"#{$b}\";\
             \n    content: \"#{$c}\";\n\
             \n    content: #{string.unquote($a)};\
             \n    content: #{string.unquote($b)};\
             \n    content: #{string.unquote($c)};\
             \n    content: \'#{string.unquote($a)}\';\
             \n    content: \'#{string.unquote($b)}\';\
             \n    content: \'#{string.unquote($c)}\';\
             \n    content: \"#{string.unquote($a)}\";\
             \n    content: \"#{string.unquote($b)}\";\
             \n    content: \"#{string.unquote($c)}\";\n\
             \n    content: #{$a + string.unquote(\"efg\")};\
             \n    content: #{$b + string.unquote(\"efg\")};\
             \n    content: #{$c + string.unquote(\"efg\")};\
             \n    content: \'#{$a + string.unquote(\"efg\")}\';\
             \n    content: \'#{$b + string.unquote(\"efg\")}\';\
             \n    content: \'#{$c + string.unquote(\"efg\")}\';\
             \n    content: \"#{$a + string.unquote(\"efg\")}\";\
             \n    content: \"#{$b + string.unquote(\"efg\")}\";\
             \n    content: \"#{$c + string.unquote(\"efg\")}\";\n\
             \n    content: #{$a + string.unquote(\"\")};\
             \n    content: #{$b + string.unquote(\"\")};\
             \n    content: #{$c + string.unquote(\"\")};\
             \n    content: \'#{$a + string.unquote(\"\")}\';\
             \n    content: \'#{$b + string.unquote(\"\")}\';\
             \n    content: \'#{$c + string.unquote(\"\")}\';\
             \n    content: \"#{$a + string.unquote(\"\")}\";\
             \n    content: \"#{$b + string.unquote(\"\")}\";\
             \n    content: \"#{$c + string.unquote(\"\")}\";\
             \n}\n"),
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
         \n}\n"
    );
}
