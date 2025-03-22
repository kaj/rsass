//! Tests auto-converted from "sass-spec/spec/directives/while.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while")
}

mod whitespace {
    use super::runner;

    mod before_var {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@while \
             \n  false {}\n"),
                ""
            );
        }
    }
}
