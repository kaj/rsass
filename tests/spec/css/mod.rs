//! Tests auto-converted from "sass-spec/spec/css"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/blockless_directive_without_semicolon.hrx"
#[test]
fn blockless_directive_without_semicolon() {
    assert_eq!(
        rsass(
            "@foo \"bar\";\
            \n"
        )
        .unwrap(),
        "@foo \"bar\";\
        \n"
    );
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
            fn cr() {
                assert_eq!(
                    rsass(
                        "/* foo\r * bar */\
            \n"
                    )
                    .unwrap(),
                    "/* foo\
        \n * bar */\
        \n"
                );
            }
            #[test]
            fn ff() {
                assert_eq!(
                    rsass(
                        "/* foo\u{c} * bar */\
            \n"
                    )
                    .unwrap(),
                    "/* foo\
        \n * bar */\
        \n"
                );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod loud {
            #[allow(unused)]
            use super::rsass;
            mod multi_line {
                #[allow(unused)]
                use super::rsass;
            }
            mod unterminated {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "scss", error tests are not supported yet.
            }
        }
    }
    mod inline {
        #[allow(unused)]
        use super::rsass;
        mod loud {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn scss() {
                assert_eq!(
                    rsass(
                        "a {\
            \n  b: c /* d */ e;\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c e;\
        \n}\
        \n"
                );
            }
        }
        mod silent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn scss() {
                assert_eq!(
                    rsass(
                        "a {\
            \n  b: c // d\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: c;\
        \n}\
        \n"
                );
            }
        }
    }
    #[test]
    fn multiple() {
        assert_eq!(
            rsass(
                ".foo {\
            \n  /* Foo Bar */\
            \n  /* Baz Bang */ }\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  /* Foo Bar */\
        \n  /* Baz Bang */\
        \n}\
        \n"
        );
    }
    #[test]
    fn multiple_stars() {
        assert_eq!(
            rsass(
                "a /***/ b {x: y}\
            \na /****/ b {x: y}\
            \na /* **/ b {x: y}\
            \na /** */ b {x: y}\
            \n"
            )
            .unwrap(),
            "a b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \na b {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    fn weird_indentation() {
        assert_eq!(
            rsass(
                ".foo {\
            \n    /* Foo\
            \n Bar\
            \nBaz */\
            \n  a: b; }\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  /* Foo\
        \n   Bar\
        \n  Baz */\
        \n  a: b;\
        \n}\
        \n"
        );
    }
}

mod custom_properties;

// From "sass-spec/spec/css/directive_with_lots_of_whitespace.hrx"
#[test]
fn directive_with_lots_of_whitespace() {
    assert_eq!(
        rsass(
            "@foo \"bar\";\
            \n"
        )
        .unwrap(),
        "@foo \"bar\";\
        \n"
    );
}

// From "sass-spec/spec/css/empty_block_directive.hrx"
#[test]
fn empty_block_directive() {
    assert_eq!(
        rsass(
            "@foo {}\
            \n"
        )
        .unwrap(),
        "@foo {}\
        \n"
    );
}

// From "sass-spec/spec/css/escape.hrx"
mod escape {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod syntax {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.
        }
    }
    #[test]
    fn zero() {
        assert_eq!(
        rsass(
            "// Although zero is not a valid code point per spec, we pass it through because\
            \n// it can be used for browser hacks.\
            \na {b: \\0}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \\0 ;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/css/function_name_identifiers.hrx"
#[test]
fn function_name_identifiers() {
    assert_eq!(
        rsass(
            "a {\
            \n  b: url;\
            \n  c: calc;\
            \n  d: element;\
            \n  e: expression;\
            \n  f: progid;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: url;\
        \n  c: calc;\
        \n  d: element;\
        \n  e: expression;\
        \n  f: progid;\
        \n}\
        \n"
    );
}

mod functions;

// From "sass-spec/spec/css/important.hrx"
mod important {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod syntax {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "eof_after_bang", error tests are not supported yet.
        }
    }
}

// From "sass-spec/spec/css/keyframes.hrx"
mod keyframes {
    #[allow(unused)]
    use super::rsass;
    mod bubble {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn empty() {
            assert_eq!(
                rsass(
                    "// Regression test for sass/dart-sass#611.\
            \na {\
            \n  @keyframes {/**/}\
            \n}\
            \n"
                )
                .unwrap(),
                "@keyframes {\
        \n  /**/\
        \n}\
        \n"
            );
        }
    }
}

mod media;

mod moz_document;

// From "sass-spec/spec/css/ms_long_filter_syntax.hrx"
#[test]
fn ms_long_filter_syntax() {
    assert_eq!(
        rsass(
            "foo {\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
        \n}\
        \n"
    );
}

mod plain;

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
            "// Attribute selector values are allowed to be unquoted as long as they\'re plain\
            \n// CSS identifiers. However, IE 11 doesn\'t recognize custom-property-style\
            \n// identifiers like `--foo` as identifiers, so they should always be quoted.\
            \n\
            \n[class=\"--foo\"], [class*=\"--foo\"] {\
            \n  x: y;\
            \n}\
            \n"
        )
        .unwrap(),
        "[class=\"--foo\"], [class*=\"--foo\"] {\
        \n  x: y;\
        \n}\
        \n"
    );
        }
        mod modifier {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn after_string() {
                assert_eq!(
                    rsass(
                        "[a=\"b\"i] {c: d}\
            \n"
                    )
                    .unwrap(),
                    "[a=b i] {\
        \n  c: d;\
        \n}\
        \n"
                );
            }
            #[test]
            fn caps() {
                assert_eq!(
                    rsass(
                        "[a=b I] {c: d}\
            \n"
                    )
                    .unwrap(),
                    "[a=b I] {\
        \n  c: d;\
        \n}\
        \n"
                );
            }
            #[test]
            fn unknown() {
                assert_eq!(
        rsass(
            "// At time of writing, only the modifiers \"i\" and \"s\" are allowed by the CSS\
            \n// spec. However, for forwards-compatibility with future CSS additions, any\
            \n// single character should be allowed.\
            \n[a=b c] {d: e}\
            \n"
        )
        .unwrap(),
        "[a=b c] {\
        \n  d: e;\
        \n}\
        \n"
    );
            }
        }
        #[test]
        fn quoted_non_identifier() {
            assert_eq!(
        rsass(
            "// Quotes should be preserved when the string they contain is not an identifier.\
            \n// See https://github.com/sass/dart-sass/issues/598.\
            \n[a=\"b.\"] {c: d}\
            \n"
        )
        .unwrap(),
        "[a=\"b.\"] {\
        \n  c: d;\
        \n}\
        \n"
    );
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
    mod inline_comments {
        #[allow(unused)]
        use super::rsass;
        mod loud {
            #[allow(unused)]
            use super::rsass;
        }
        mod silent {
            #[allow(unused)]
            use super::rsass;
        }
    }
    mod placeholder {
        #[allow(unused)]
        use super::rsass;
        mod pseudoselectors {
            #[allow(unused)]
            use super::rsass;
            mod matches {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn solo() {
                    assert_eq!(
        rsass(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
            \n// removed.\
            \na:matches(%b) {x: y}\
            \n"
        )
        .unwrap(),
        ""
    );
                }
                #[test]
                #[ignore] // wrong result
                fn with_real() {
                    assert_eq!(
        rsass(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
            \na:matches(%b, c) {x: y}\
            \n"
        )
        .unwrap(),
        "a:matches(c) {\
        \n  x: y;\
        \n}\
        \n"
    );
                }
            }
            mod not {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn solo() {
                    assert_eq!(
        rsass(
            "// Since `%b` doesn\'t exist, all `a` elements match `a:not(%b)`.\
            \na:not(%b) {x: y}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  x: y;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn universal() {
                    assert_eq!(
        rsass(
            "// Since `%b` doesn\'t exist, all elements match `:not(%b)`.\
            \n:not(%b) {x: y}\
            \n"
        )
        .unwrap(),
        "* {\
        \n  x: y;\
        \n}\
        \n"
    );
                }
                #[test]
                #[ignore] // wrong result
                fn with_real() {
                    assert_eq!(
        rsass(
            "// Since `%b` doesn\'t exist, it can be removed from the `:not` pseudoselector.\
            \na:not(%b, c) {x: y}\
            \n"
        )
        .unwrap(),
        "a:not(c) {\
        \n  x: y;\
        \n}\
        \n"
    );
                }
            }
        }
    }
    mod pseudoselector {
        #[allow(unused)]
        use super::rsass;
        mod nested {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn adjacent_combinators() {
                assert_eq!(
                    rsass(
                        "// Regression test for sass/dart-sass#1038\
            \na {\
            \n  b:c, > d {x: y}\
            \n}\
            \n"
                    )
                    .unwrap(),
                    "a b:c, a > d {\
        \n  x: y;\
        \n}\
        \n"
                );
            }
        }
    }

    // Ignoring "reference_combinator", error tests are not supported yet.
    #[test]
    #[ignore] // unexepected error
    fn slotted() {
        assert_eq!(
            rsass(
                "::slotted(.a) {x: y}\
            \n\
            \n::slotted(.c.d) {x: y}\
            \n.e {@extend .c}\
            \n\
            \n::slotted(.f) {x: y}\
            \n::slotted(.g) {@extend .f}\
            \n"
            )
            .unwrap(),
            "::slotted(.a) {\
        \n  x: y;\
        \n}\
        \n::slotted(.c.d, .d.e) {\
        \n  x: y;\
        \n}\
        \n::slotted(.f, ::slotted(.g)) {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
}

mod supports;

mod unicode_range;

mod unknown_directive;
