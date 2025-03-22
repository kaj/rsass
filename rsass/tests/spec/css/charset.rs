//! Tests auto-converted from "sass-spec/spec/css/charset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("charset")
}

mod whitespace {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@charset\
             \n  \"a\";\n"),
            ""
        );
    }
}
