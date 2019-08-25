//! Tests auto-converted from "sass-spec/spec/css"
//! version cb1ab541, 2019-08-16 17:59:57 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["plain"]
use rsass::{compile_scss, OutputStyle};

// From "sass-spec/spec/css/blockless_directive_without_semicolon.hrx"
#[test]
fn blockless_directive_without_semicolon() {
    assert_eq!(rsass("@foo \"bar\";\n").unwrap(), "@foo \"bar\";\n");
}

// From "sass-spec/spec/css/comment.hrx"
mod comment {
    #[allow(unused)]
    use super::rsass;
    mod converts_newlines {
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
            #[ignore] // failing
            fn cr() {
                assert_eq!(
                    rsass("/* foo\r * bar */\n").unwrap(),
                    "/* foo\n * bar */\n"
                );
            }
            #[test]
            #[ignore] // failing
            fn ff() {
                assert_eq!(
                    rsass("/* foo\u{c} * bar */\n").unwrap(),
                    "/* foo\n * bar */\n"
                );
            }
        }
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(".foo {\n  /* Foo Bar */\n  /* Baz Bang */ }\n").unwrap(),
            ".foo {\n  /* Foo Bar */\n  /* Baz Bang */\n}\n"
        );
    }
    #[test]
    fn multiple_stars() {
        assert_eq!(
        rsass(
            "a /***/ b {x: y}\na /****/ b {x: y}\na /* **/ b {x: y}\na /** */ b {x: y}\n"
        )
        .unwrap(),
        "a b {\n  x: y;\n}\na b {\n  x: y;\n}\na b {\n  x: y;\n}\na b {\n  x: y;\n}\n"
    );
    }
    #[test]
    fn weird_indentation() {
        assert_eq!(
            rsass(".foo {\n    /* Foo\n Bar\nBaz */\n  a: b; }\n").unwrap(),
            ".foo {\n  /* Foo\n Bar\nBaz */\n  a: b;\n}\n"
        );
    }
}

mod custom_properties;

// From "sass-spec/spec/css/directive_with_lots_of_whitespace.hrx"
#[test]
fn directive_with_lots_of_whitespace() {
    assert_eq!(rsass("@foo \"bar\";\n").unwrap(), "@foo \"bar\";\n");
}

// From "sass-spec/spec/css/empty_block_directive.hrx"
#[test]
fn empty_block_directive() {
    assert_eq!(rsass("@foo {}\n").unwrap(), "@foo {}\n");
}

// From "sass-spec/spec/css/function_name_identifiers.hrx"
#[test]
fn function_name_identifiers() {
    assert_eq!(
        rsass(
            "a {\n  b: url;\n  c: calc;\n  d: element;\n  e: expression;\n  f: progid;\n}\n"
        )
        .unwrap(),
        "a {\n  b: url;\n  c: calc;\n  d: element;\n  e: expression;\n  f: progid;\n}\n"
    );
}

// From "sass-spec/spec/css/keyframes.hrx"
mod keyframes {
    #[allow(unused)]
    use super::rsass;
    mod bubble {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn empty() {
            assert_eq!(
        rsass(
            "// Regression test for sass/dart-sass#611.\na {\n  @keyframes {/**/}\n}\n"
        )
        .unwrap(),
        "@keyframes {\n  /**/\n}\n"
    );
        }
    }
}

mod media;

mod min_max;

mod moz_document;

// From "sass-spec/spec/css/ms_long_filter_syntax.hrx"
#[test]
#[ignore] // failing
fn ms_long_filter_syntax() {
    assert_eq!(
        rsass(
            "foo {\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000); }\n"
        )
        .unwrap(),
        "foo {\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n}\n"
    );
}

// Ignoring "plain", not expected to work yet.

// From "sass-spec/spec/css/selector.hrx"
mod selector {
    #[allow(unused)]
    use super::rsass;
    mod attribute {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn dash_dash() {
            assert_eq!(
        rsass(
            "// Attribute selector values are allowed to be unquoted as long as they\'re plain\n// CSS identifiers. However, IE 11 doesn\'t recognize custom-property-style\n// identifiers like `--foo` as identifiers, so they should always be quoted.\n\n[class=\"--foo\"], [class*=\"--foo\"] {\n  x: y;\n}\n"
        )
        .unwrap(),
        "[class=\"--foo\"], [class*=\"--foo\"] {\n  x: y;\n}\n"
    );
        }
        mod modifier {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // failing
            fn after_string() {
                assert_eq!(
                    rsass("[a=\"b\"i] {c: d}\n").unwrap(),
                    "[a=\"b\" i] {\n  c: d;\n}\n"
                );
            }
            #[test]
            fn caps() {
                assert_eq!(
                    rsass("[a=b I] {c: d}\n").unwrap(),
                    "[a=b I] {\n  c: d;\n}\n"
                );
            }
            #[test]
            fn unknown() {
                assert_eq!(
        rsass(
            "// At time of writing, only the modifiers \"i\" and \"s\" are allowed by the CSS\n// spec. However, for forwards-compatibility with future CSS additions, any\n// single character should be allowed.\n[a=b c] {d: e}\n"
        )
        .unwrap(),
        "[a=b c] {\n  d: e;\n}\n"
    );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod attribute {
            #[allow(unused)]
            use super::rsass;
            mod modifier {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "digit", error tests are not supported yet.

                // Ignoring "no_operator", error tests are not supported yet.

                // Ignoring "too_long", error tests are not supported yet.

                // Ignoring "underscore", error tests are not supported yet.

                // Ignoring "unicode", error tests are not supported yet.
            }
        }
    }

    // Ignoring "reference_combinator", error tests are not supported yet.
    #[test]
    #[ignore] // failing
    fn slotted() {
        assert_eq!(
        rsass(
            "::slotted(.a) {x: y}\n\n::slotted(.c.d) {x: y}\n.e {@extend .c}\n\n::slotted(.f) {x: y}\n::slotted(.g) {@extend .f}\n"
        )
        .unwrap(),
        "::slotted(.a) {\n  x: y;\n}\n::slotted(.c.d, .d.e) {\n  x: y;\n}\n::slotted(.f, ::slotted(.g)) {\n  x: y;\n}\n"
    );
    }
}

mod unicode_range;

mod unknown_directive;

// From "sass-spec/spec/css/url.hrx"
mod url {
    #[allow(unused)]
    use super::rsass;
    mod exclam {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn middle() {
            assert_eq!(
                rsass("a {b: url(http://c.d/e!f)}\n").unwrap(),
                "a {\n  b: url(http://c.d/e!f);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn only() {
            assert_eq!(
                rsass("a {b: url(!)}\n").unwrap(),
                "a {\n  b: url(!);\n}\n"
            );
        }
    }
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
