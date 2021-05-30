//! Tests auto-converted from "sass-spec/spec/css/selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod attribute {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn dash_dash() {
        assert_eq!(
        runner().ok(
            "// Attribute selector values are allowed to be unquoted as long as they\'re plain\
             \n// CSS identifiers. However, IE 11 doesn\'t recognize custom-property-style\
             \n// identifiers like `--foo` as identifiers, so they should always be quoted.\n\
             \n[class=\"--foo\"], [class*=\"--foo\"] {\
             \n  x: y;\
             \n}\n"
        ),
        "[class=\"--foo\"], [class*=\"--foo\"] {\
         \n  x: y;\
         \n}\n"
    );
    }
    mod modifier {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after_string() {
            assert_eq!(
                runner().ok("[a=\"b\"i] {c: d}\n"),
                "[a=b i] {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn caps() {
            assert_eq!(
                runner().ok("[a=b I] {c: d}\n"),
                "[a=b I] {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
        runner().ok(
            "// At time of writing, only the modifiers \"i\" and \"s\" are allowed by the CSS\
             \n// spec. However, for forwards-compatibility with future CSS additions, any\
             \n// single character should be allowed.\
             \n[a=b c] {d: e}\n"
        ),
        "[a=b c] {\
         \n  d: e;\
         \n}\n"
    );
        }
    }
    #[test]
    fn quoted_non_identifier() {
        assert_eq!(
        runner().ok(
            "// Quotes should be preserved when the string they contain is not an identifier.\
             \n// See https://github.com/sass/dart-sass/issues/598.\
             \n[a=\"b.\"] {c: d}\n"
        ),
        "[a=\"b.\"] {\
         \n  c: d;\
         \n}\n"
    );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod attribute {
        #[allow(unused)]
        use super::runner;

        mod modifier {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn digit() {
                assert_eq!(
        runner().err(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b 1] {c: d}\n"
        ),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b 1]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn no_operator() {
                assert_eq!(
                    runner().err("[a b] {c: d}\n"),
                    "Error: Expected \"]\".\
         \n  ,\
         \n1 | [a b]{c: d}\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet",
                );
            }
            #[test]
            #[ignore] // wrong error
            fn too_long() {
                assert_eq!(
                    runner().err(
                        "// Attribute modifiers must be single characters.\
             \n[a=b cd] {e: f}\n"
                    ),
                    "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b cd]{e: f}\
         \n  |       ^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
            #[test]
            #[ignore] // wrong error
            fn underscore() {
                assert_eq!(
        runner().err(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b _] {c: d}\n"
        ),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b _]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn unicode() {
                assert_eq!(
        runner().err(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b ï] {c: d}\n"
        ),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b ï]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
        }
    }
}
mod escaping {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn dollar_char() {
        assert_eq!(
            runner().ok(".u\\$ {a: b;}\n"),
            ".u\\$ {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn dollar_char_as_numeric() {
        assert_eq!(
            runner().ok(".u\\24 {a: b;}\n"),
            ".u\\$ {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number_as_first_char_with_space() {
        assert_eq!(
            runner().ok(".\\31 u {a: b;}\n"),
            ".\\31 u {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number_as_first_char_without_space() {
        assert_eq!(
            runner().ok(".\\31u {a: b;}\n"),
            ".\\31 u {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn number_as_nonfirst_char_with_space() {
        assert_eq!(
            runner().ok(".a\\31 u {a: b;}\n"),
            ".a1u {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn number_as_nonfirst_char_without_space() {
        assert_eq!(
            runner().ok(".a\\31u {a: b;}\n"),
            ".a1u {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn parenthesis_in_interpolation() {
        assert_eq!(
            runner().ok(".u#{\'\\\\28\'} { a: b; }\n"),
            ".u\\( {\
         \n  a: b;\
         \n}\n"
        );
    }
}
mod inline_comments {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;
    }
    mod silent {
        #[allow(unused)]
        use super::runner;
    }
}
mod placeholder {
    #[allow(unused)]
    use super::runner;

    mod pseudoselectors {
        #[allow(unused)]
        use super::runner;

        mod is {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn solo() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
             \n// removed.\
             \na:is(%b) {x: y}\n"
        ),
        ""
    );
            }
            #[test]
            #[ignore] // wrong result
            fn with_real() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
             \na:is(%b, c) {x: y}\n"
        ),
        "a:is(c) {\
         \n  x: y;\
         \n}\n"
    );
            }
        }
        mod matches {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn solo() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
             \n// removed.\
             \na:matches(%b) {x: y}\n"
        ),
        ""
    );
            }
            #[test]
            #[ignore] // wrong result
            fn with_real() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
             \na:matches(%b, c) {x: y}\n"
        ),
        "a:matches(c) {\
         \n  x: y;\
         \n}\n"
    );
            }
        }
        mod not {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn solo() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, all `a` elements match `a:not(%b)`.\
             \na:not(%b) {x: y}\n"
        ),
        "a {\
         \n  x: y;\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn universal() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, all elements match `:not(%b)`.\
             \n:not(%b) {x: y}\n"
        ),
        "* {\
         \n  x: y;\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn with_real() {
                assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, it can be removed from the `:not` pseudoselector.\
             \na:not(%b, c) {x: y}\n"
        ),
        "a:not(c) {\
         \n  x: y;\
         \n}\n"
    );
            }
        }
    }
}
mod pseudoselector {
    #[allow(unused)]
    use super::runner;

    mod nested {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn adjacent_combinators() {
            assert_eq!(
                runner().ok("// Regression test for sass/dart-sass#1038\
             \na {\
             \n  b:c, > d {x: y}\
             \n}\n"),
                "a b:c, a > d {\
         \n  x: y;\
         \n}\n"
            );
        }
    }
}
#[test]
#[ignore] // wrong error
fn reference_combinator() {
    assert_eq!(
        runner().err(
            "// Reference combinators used to be supported by Sass when they were part of the\
             \n// CSS spec, but they\'re no longer supported and should now produce errors.\
             \n.foo /bar/ .baz {\
             \n  a: b;\
             \n}\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n3 | .foo /bar/ .baz{\
         \n  |      ^\
         \n  \'\
         \n  input.scss 3:6  root stylesheet",
    );
}
#[test]
#[ignore] // wrong result
fn slotted() {
    assert_eq!(
        runner().ok("::slotted(.a) {x: y}\n\
             \n::slotted(.c.d) {x: y}\
             \n.e {@extend .c}\n\
             \n::slotted(.f) {x: y}\
             \n::slotted(.g) {@extend .f}\n"),
        "::slotted(.a) {\
         \n  x: y;\
         \n}\
         \n::slotted(.c.d, .d.e) {\
         \n  x: y;\
         \n}\
         \n::slotted(.f, ::slotted(.g)) {\
         \n  x: y;\
         \n}\n"
    );
}
