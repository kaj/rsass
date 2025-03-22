//! Tests auto-converted from "sass-spec/spec/css/plain/import/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod supports {
    use super::runner;

    mod declaration {
        use super::runner;

        mod prop {
            use super::runner;

            mod after_open {
                use super::runner;

                #[test]
                fn scss() {
                    assert_eq!(
                        runner().ok("@import \"a.css\" supports(\
             \n  a: b)\n"),
                        "@import \"a.css\" supports(a: b);\n"
                    );
                }
            }
            mod space_after_open {
                use super::runner;

                #[test]
                fn scss() {
                    assert_eq!(
                        runner().ok("@import \"a.css\" supports( a: b)\n"),
                        "@import \"a.css\" supports(a: b);\n"
                    );
                }
            }
        }
    }
}
