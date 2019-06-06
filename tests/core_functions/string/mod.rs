//! Tests auto-converted from "sass-spec/spec/core_functions/string"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/string/quote.hrx"
mod quote {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "argument_type", error tests are not supported yet.
    }
    mod quote_unquoted_quote {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn double() {
            assert_eq!(
        rsass(
            "// See sass/libsass#2873\na {b: quote(unquote(\'\"\') + unquote(\"\'\"))}\n"
        )
        .unwrap(),
        "a {\n  b: \"\\\"\'\";\n}\n"
    );
        }
        #[test]
        #[ignore] // failing
        fn single() {
            assert_eq!(
        rsass("// See sass/libsass#2873\na {b: quote(unquote(\'\"\'))}\n")
            .unwrap(),
        "a {\n  b: \'\"\';\n}\n"
    );
        }
    }
    #[test]
    fn quoted_double() {
        assert_eq!(
            rsass("a {b: quote(\"c\")}\n").unwrap(),
            "a {\n  b: \"c\";\n}\n"
        );
    }
    #[test]
    fn quoted_single() {
        assert_eq!(
            rsass("a {b: quote(\'c\')}\n").unwrap(),
            "a {\n  b: \"c\";\n}\n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            rsass("a {b: quote(c)}\n").unwrap(),
            "a {\n  b: \"c\";\n}\n"
        );
    }
}
