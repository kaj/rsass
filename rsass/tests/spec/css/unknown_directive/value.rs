//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/value.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value")
}

mod url {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn escaped() {
        assert_eq!(
            runner().ok("@a \\75 rl(http://example.org);\n"),
            "@a \\75 rl(http://example.org);\n"
        );
    }
}
