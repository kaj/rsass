//! Tests auto-converted from "sass-spec/spec/css/custom_properties"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/custom_properties/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;
    mod brackets {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "curly", error tests are not supported yet.

        // Ignoring "curly_in_square", error tests are not supported yet.

        // Ignoring "paren", error tests are not supported yet.

        // Ignoring "paren_in_curly", error tests are not supported yet.

        // Ignoring "square", error tests are not supported yet.

        // Ignoring "square_in_paren", error tests are not supported yet.
    }

    // Ignoring "empty", error tests are not supported yet.

    // Ignoring "empty_interpolation", error tests are not supported yet.
}

// From "sass-spec/spec/css/custom_properties/exclamation.hrx"
#[test]
#[ignore] // unexepected error
fn exclamation() {
    assert_eq!(
        rsass(
            ".exclamation {\
            \n  // `!` is technically not allowed at the top-level of a custom property, but\
            \n  // that\'s only because `!important` is filtered out before the custom property\
            \n  // is parsed by the CSS parser. As far as Sass is concerned, it\'s fine.\
            \n  --important: value !important;\
            \n\
            \n  // We even allow constructions like these for forwards-compatibility with\
            \n  // additional flags or syntax CSS might add.\
            \n  --multiple: !important !important;\
            \n  --other-word: !something;\
            \n  --in-identifier: foo!bar;\
            \n  --just-exclam: !;\
            \n  --just-exclams: !!!!!!!;\
            \n}\
            \n"
        )
        .unwrap(),
        ".exclamation {\
        \n  --important: value !important;\
        \n  --multiple: !important !important;\
        \n  --other-word: !something;\
        \n  --in-identifier: foo!bar;\
        \n  --just-exclam: !;\
        \n  --just-exclams: !!!!!!!;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/indentation.hrx"
#[test]
#[ignore] // unexepected error
fn indentation() {
    assert_eq!(
        rsass(
            ".indentation {\
            \n  --simple: {\
            \n    foo: bar;\
            \n  };\
            \n\
            \n  --empty-line: {\
            \n    foo: bar;\
            \n\
            \n    baz: bang;\
            \n  };\
            \n\
            \n  --multi-level: {\
            \n   one\
            \n    two\
            \n     three\
            \n      four\
            \n  };\
            \n\
            \n  --all-indented: {\
            \n    foo: bar; };\
            \n\
            \n  --below-base:\
            \n    foo\
            \n bar\
            \n   baz;\
            \n\
            \n         --deep-base: {\
            \n           foo: bar;\
            \n         };\
            \n\
            \n\t--hard-tabs: {\
            \n\t\tfoo: bar;\
            \n\t};\
            \n}\
            \n"
        )
        .unwrap(),
        ".indentation {\
        \n  --simple: {\
        \n    foo: bar;\
        \n  };\
        \n  --empty-line: {\
        \n    foo: bar;\
        \n    baz: bang;\
        \n  };\
        \n  --multi-level: {\
        \n   one\
        \n    two\
        \n     three\
        \n      four\
        \n  };\
        \n  --all-indented: {\
        \n    foo: bar; };\
        \n  --below-base:\
        \n     foo\
        \n  bar\
        \n    baz;\
        \n  --deep-base: {\
        \n    foo: bar;\
        \n  };\
        \n  --hard-tabs: {\
        \n  \tfoo: bar;\
        \n  };\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/name_interpolation.hrx"
#[test]
#[ignore] // wrong result
fn name_interpolation() {
    assert_eq!(
        rsass(
            ".name-interpolation {\
            \n  // If the entire name is interpolated, SassScript is allowed on the\
            \n  // right-hand side because we don\'t know it\'s a custom property at parse time.\
            \n  #{--entire}: 1 + 2;\
            \n\
            \n  // Same if the first hyphen is interpolated.\
            \n  -#{-first-hyphen}: 1 + 2;\
            \n\
            \n  // But if the name is interpolated, the right-hand side is static.\
            \n  --#{only-name}: 1 + 2;\
            \n  // However, interpolation is still allowed on the right-hand side.\
            \n  --#{only-name-interp-value}: #{1 + 2};\
            \n\
            \n  // The name can also be partially interpolated.\
            \n  --#{initial}-interp: 1 + 2;\
            \n  --midd#{le-int}erp: 1 + 2;\
            \n  --final-#{interp}: 1 + 2;\
            \n  --#{doub}le-int#{erp}: 1 + 2;\
            \n}\
            \n"
        )
        .unwrap(),
        ".name-interpolation {\
        \n  --entire: 3;\
        \n  --first-hyphen: 3;\
        \n  --only-name: 1 + 2;\
        \n  --only-name-interp-value: 3;\
        \n  --initial-interp: 1 + 2;\
        \n  --middle-interp: 1 + 2;\
        \n  --final-interp: 1 + 2;\
        \n  --double-interp: 1 + 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/nesting_characters.hrx"
#[test]
#[ignore] // unexepected error
fn nesting_characters() {
    assert_eq!(
        rsass(
            ".nesting-characters {\
            \n  --parens: (foo; (bar: baz;) bang!);\
            \n  --curly: {foo; {bar: baz;} bang!};\
            \n  --square: [foo; [bar: baz;] bang!];\
            \n  --multiple: [({{([])}})];\
            \n\
            \n  // Nested properties aren\'t supported in custom properties.\
            \n  --nested-props: {foo: bar;};\
            \n\
            \n  // A property that\'s ambiguous with a nested selector is interpreted as a\
            \n  // custom property.\
            \n  --ambiguous:foo {bar: baz;};\
            \n}\
            \n"
        )
        .unwrap(),
        ".nesting-characters {\
        \n  --parens: (foo; (bar: baz;) bang!);\
        \n  --curly: {foo; {bar: baz;} bang!};\
        \n  --square: [foo; [bar: baz;] bang!];\
        \n  --multiple: [({{([])}})];\
        \n  --nested-props: {foo: bar;};\
        \n  --ambiguous:foo {bar: baz;};\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/script.hrx"
#[test]
#[ignore] // unexepected error
fn script() {
    assert_eq!(
        rsass(
            ".script {\
            \n  // All script values except interpolation are interpreted statically.\
            \n  --variable: $variable;\
            \n  --operator: 1 + 1;\
            \n  --function: red(#ffffff);\
            \n  --list: (a b c);\
            \n  --map: (a: b, c: d);\
            \n}\
            \n"
        )
        .unwrap(),
        ".script {\
        \n  --variable: $variable;\
        \n  --operator: 1 + 1;\
        \n  --function: red(#ffffff);\
        \n  --list: (a b c);\
        \n  --map: (a: b, c: d);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/simple.hrx"
#[test]
#[ignore] // unexepected error
fn simple() {
    assert_eq!(
        rsass(
            ".simple {\
            \n  --single: value;\
            \n  --multiple: value1 value2;\
            \n  --function: foo(bar);\
            \n  --url: url(http://foo.com/bar);\
            \n  --color: #foo;\
            \n  --exponent: 12.6e7;\
            \n  --close-comment: */;\
            \n\
            \n  // The whitespace here DOES count as a token and needs to be preserved.\
            \n  --empty: ;\
            \n\
            \n  // Single-line comments are not supported in variables.\
            \n  --single-line: // (\
            \n    );\
            \n\
            \n  // Extra whitespace isn\'t added.\
            \n  --no-extra-whitespace:value;\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple {\
        \n  --single: value;\
        \n  --multiple: value1 value2;\
        \n  --function: foo(bar);\
        \n  --url: url(http://foo.com/bar);\
        \n  --color: #foo;\
        \n  --exponent: 12.6e7;\
        \n  --close-comment: */;\
        \n  --empty: ;\
        \n  --single-line: // (\
        \n    );\
        \n  --no-extra-whitespace:value;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/strings.hrx"
#[test]
fn strings() {
    assert_eq!(
        rsass(
            ".strings {\
            \n  // Strings are tokenized as units, so their contents shouldn\'t affect anything\
            \n  // else.\
            \n  --text: \"foo\";\
            \n  --bang: \"!\";\
            \n  --semicolon: \";\";\
            \n  --square: \"][\";\
            \n  --curly: \"}{\";\
            \n  --parens: \")(\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".strings {\
        \n  --text: \"foo\";\
        \n  --bang: \"!\";\
        \n  --semicolon: \";\";\
        \n  --square: \"][\";\
        \n  --curly: \"}{\";\
        \n  --parens: \")(\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/syntax.hrx"
mod syntax {
    #[allow(unused)]
    use super::rsass;
    mod double_dash {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn declare() {
            assert_eq!(
                rsass(
                    "a {--: b}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  --: b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn test_use() {
            assert_eq!(
                rsass(
                    "a {b: var(--)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: var(--);\
        \n}\
        \n"
            );
        }
    }
    mod initial_digit {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn declare() {
            assert_eq!(
                rsass(
                    "a {--1: b}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  --1: b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn test_use() {
            assert_eq!(
                rsass(
                    "a {b: var(--1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: var(--1);\
        \n}\
        \n"
            );
        }
    }
    mod triple_dash {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn declare() {
            assert_eq!(
                rsass(
                    "a {---: b}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  ---: b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn test_use() {
            assert_eq!(
                rsass(
                    "a {b: var(---)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: var(---);\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/css/custom_properties/trailing_comment.hrx"
mod trailing_comment {
    #[allow(unused)]
    use super::rsass;
    mod sass {
        #[allow(unused)]
        use super::rsass;
    }
    mod scss {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn loud() {
            assert_eq!(
                rsass(
                    "a {\
            \n  --b: c /* comment */;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  --b: c /* comment */;\
        \n}\
        \n"
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                rsass(
                    "a {\
            \n  --b: c // comment;\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  --b: c // comment;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/css/custom_properties/trailing_whitespace.hrx"
#[test]
#[ignore] // wrong result
fn trailing_whitespace() {
    assert_eq!(
        rsass(
            ".trailing-whitespace {\
            \n  --space: value ;\
            \n  --tab: value\t;\
            \n  --newline: value\
            \n;\
            \n  --before-closing-brace: value\
            \n}\
            \n"
        )
        .unwrap(),
        ".trailing-whitespace {\
        \n  --space: value ;\
        \n  --tab: value\t;\
        \n  --newline: value ;\
        \n  --before-closing-brace: value ;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/value_interpolation.hrx"
#[test]
fn value_interpolation() {
    assert_eq!(
        rsass(
            ".value-interpolation {\
            \n  // Interpolation is the only Sass construct that\'s supported in custom\
            \n  // variables.\
            \n  --alone: #{1 + 2};\
            \n  --in-list: a #{1 + 2} c;\
            \n  --in-ident: foo#{1 + 2}bar;\
            \n  --in-string: \"foo#{1 + 2}bar\";\
            \n  --in-uri: uri(foo#{1 + 2}bar);\
            \n}\
            \n"
        )
        .unwrap(),
        ".value-interpolation {\
        \n  --alone: 3;\
        \n  --in-list: a 3 c;\
        \n  --in-ident: foo3bar;\
        \n  --in-string: \"foo3bar\";\
        \n  --in-uri: uri(foo3bar);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/custom_properties/without_semicolon.hrx"
#[test]
#[ignore] // wrong result
fn without_semicolon() {
    assert_eq!(
        rsass(
            ".simple-value {\
            \n  // A custom property at the end of a style rule doesn\'t need a semicolon.\
            \n  --without-semicolon: value\
            \n}\
            \n\
            \n.bracketed-value {\
            \n  --without-semicolon: {\
            \n    a: b\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple-value {\
        \n  --without-semicolon: value ;\
        \n}\
        \n.bracketed-value {\
        \n  --without-semicolon: {\
        \n    a: b\
        \n  } ;\
        \n}\
        \n"
    );
}
