//! These are from the `css` directory in the sass specification.
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

#[test]
fn multi_star_comments() {
    // Note, actual expected has single newlines, but the sass-spec
    // test runner succeeds my implementation.
    check(
        "a /***/ b {x: y}\n\
         a /****/ b {x: y}\n\
         a /* **/ b {x: y}\n\
         a /** */ b {x: y}\n",
        "a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n\n\
         a b {\n  x: y;\n}\n",
    )
}

#[test]
fn unknown_directive() {
    check(
        "// Unknown directives should support almost any sequence of \
         valid tokens,\n\
         // including interpolation.\n\n\
         // By default, characters are passed through unaltered.\n\
         @asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n\n\
         // Strings are tokenized as strings.\n\
         @asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n\n\
         // Comments are preserved.\n\
         @asdf foo //\n      bar;\n\
         @asdf foo /* bar */ baz;\n\n\
         // Interpolation is supported in plain text, strings, and URLs.\n\
         @asdf #{1 + 2};\n\
         @asdf \"foo #{\"bar\"} baz\";\n\
         @asdf 'foo #{'bar'} baz';\n\
         @asdf url(http://#{\")\"}.com/);\n\
         @asdf url(\"http://#{\")\"}.com/\");\n",
        "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n\
         @asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n\
         @asdf foo //\n      bar;\n\
         @asdf foo /* bar */ baz;\n\
         @asdf 3;\n\
         @asdf \"foo bar baz\";\n\
         @asdf 'foo bar baz';\n\
         @asdf url(http://).com/);\n\
         @asdf url(\"http://).com/\");\n",
    )
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_bytes(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
