//! Parts of `sass_spec/spec/css/unknown_directive` as separate tests.
//!
//! Unknown directives should support almost any sequence of valid tokens,
//! including interpolation.
use super::check;

#[test]
fn t01_characters_are_passed_through_unaltered() {
    check(
        "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n",
        "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n",
    )
}
#[test]
fn t02_strings_are_tokenized_as_strings() {
    check(
        "@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n",
        "@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n",
    )
}
#[test]
fn t03_comments_are_preserved() {
    check(
        "@asdf foo //\n      bar;\n",
        "@asdf foo //\n      bar;\n",
    )
}
#[test]
fn t04_comments_are_preserved() {
    check(
        "@asdf foo /* bar */ baz;",
        "@asdf foo /* bar */ baz;\n",
    )
}
#[test]
fn t05_interpolation_plain() {
    check("@asdf #{1 + 2};\n", "@asdf 3;\n")
}
#[test]
fn t06_interpolation_in_string() {
    check(
        "@asdf \"foo #{\"bar\"} baz\";\n",
        "@asdf \"foo bar baz\";\n",
    )
}
#[test]
fn t07_interpolation_in_string() {
    check(
        "@asdf 'foo #{'bar'} baz';\n",
        "@asdf 'foo bar baz';\n",
    )
}
#[test]
fn t08_interpolation_in_url() {
    check(
        "@asdf url(http://#{\")\"}.com/);\n",
        "@asdf url(http://).com/);\n",
    )
}
#[test]
fn t09_interpolation_in_url() {
    check(
        "@asdf url(\"http://#{\")\"}.com/\");\n",
        "@asdf url(\"http://).com/\");\n",
    )
}
