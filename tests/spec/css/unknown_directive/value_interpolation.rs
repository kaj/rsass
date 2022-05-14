//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/value_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// Unknown directives should support interpolation in plain text, strings,\
             \n// identifiers, and URLs.\
             \n@asdf #{1 + 2};\n\
             \n@asdf foo#{\"bar\"}baz;\n\
             \n@asdf \"foo #{\"bar\"} baz\";\n\
             \n@asdf \'foo #{\'bar\'} baz\';\n\
             \n@asdf url(http://#{\")\"}.com/);\n\
             \n@asdf url(\"http://#{\")\"}.com/\");\n"
        ),
        "@asdf 3;\
         \n@asdf foobarbaz;\
         \n@asdf \"foo bar baz\";\
         \n@asdf \"foo bar baz\";\
         \n@asdf url(http://).com/);\
         \n@asdf url(\"http://).com/\");\n"
    );
}
