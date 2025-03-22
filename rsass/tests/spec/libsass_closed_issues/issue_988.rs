//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_988.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_988")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@function str-replace($string, $search, $replace: \'\') {\
             \n  $index: string.index($string, $search);\
             \n  @if $index {\
             \n    @return string.slice($string, 1, $index - 1) + $replace +\
             \n      str-replace(string.slice($string, $index + string.length($search)), $search, $replace);\
             \n  }\
             \n  @return $string;\
             \n}\n\
             \n$string: \'Foo Bar Baz Qux\';\
             \n.foo {\
             \n  content: str-replace($string, \' \', \'-\');\
             \n}"
        ),
        ".foo {\
         \n  content: \"Foo-Bar-Baz-Qux\";\
         \n}\n"
    );
}
