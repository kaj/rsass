//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_988.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function str-replace($string, $search, $replace: \'\') {\
            \n  $index: str-index($string, $search);\
            \n  @if $index {\
            \n    @return str-slice($string, 1, $index - 1) + $replace +\
            \n      str-replace(str-slice($string, $index + str-length($search)), $search, $replace);\
            \n  }\
            \n  @return $string;\
            \n}\
            \n\
            \n$string: \'Foo Bar Baz Qux\';\
            \n.foo {\
            \n  content: str-replace($string, \' \', \'-\');\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"Foo-Bar-Baz-Qux\";\
        \n}\
        \n"
    );
}
