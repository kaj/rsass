//! Tests auto-converted from "sass-spec/spec/css/selector/attribute.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("attribute")
}

#[test]
fn backslash() {
    assert_eq!(
        runner().ok(
            "// Regression test for https://github.com/sass/dart-sass/issues/1423.\
             \n[a=\"\\\\\"] {c: d}\n"
        ),
        "[a=\"\\\\\"] {\
         \n  c: d;\
         \n}\n"
    );
}
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
#[test]
fn empty_namespace() {
    assert_eq!(
        runner().ok("[|a] {a: b;}\n"),
        "[|a] {\
         \n  a: b;\
         \n}\n"
    );
}
mod error {
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
