//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/separate_file.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("separate_file")
        .mock_file("direct/_config.scss", "$a: configured;\n")
        .mock_file("direct/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file(
            "direct/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("nested/through_forward/_config.scss", "$a: configured;\n")
        .mock_file(
            "nested/through_forward/_config_wrapper.scss",
            "@forward \"config\";\n",
        )
        .mock_file(
            "nested/through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "nested/through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "shadowing/nested/global/through_forward/_config.scss",
            "$a: configured;\n",
        )
        .mock_file(
            "shadowing/nested/global/through_forward/_config_wrapper.scss",
            "@forward \"config\";\n",
        )
        .mock_file(
            "shadowing/nested/global/through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "shadowing/nested/global/through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "shadowing/nested/local/through_forward/_config.scss",
            "$a: configured;\n",
        )
        .mock_file(
            "shadowing/nested/local/through_forward/_config_wrapper.scss",
            "@forward \"config\";\n",
        )
        .mock_file(
            "shadowing/nested/local/through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "shadowing/nested/local/through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "shadowing/through_forward/_config.scss",
            "$a: configured;\n",
        )
        .mock_file(
            "shadowing/through_forward/_config_wrapper.scss",
            "@forward \"config\";\n",
        )
        .mock_file(
            "shadowing/through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "shadowing/through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("through_forward/_config.scss", "$a: configured;\n")
        .mock_file(
            "through_forward/_config_wrapper.scss",
            "@forward \"config\";\n",
        )
        .mock_file(
            "through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
#[ignore] // wrong result
fn direct() {
    let runner = runner().with_cwd("direct");
    assert_eq!(
        runner.ok("@import \"config\";\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
mod nested {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nested")
    }

    #[test]
    #[ignore] // wrong result
    fn through_forward() {
        let runner = runner().with_cwd("through_forward");
        assert_eq!(
            runner.ok("// Regression test for sass/dart-sass#2641\
             \na {\
             \n  @import \"config_wrapper\";\
             \n  @import \"midstream\";\
             \n}\n"),
            "a b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
mod shadowing {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("shadowing")
    }

    mod nested {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("nested")
        }

        mod global {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("global")
            }

            #[test]
            #[ignore] // wrong result
            fn through_forward() {
                let runner = runner().with_cwd("through_forward");
                assert_eq!(
                    runner.ok("// Regression test for sass/dart-sass#2641\
             \n$a: shadowed;\
             \na {\
             \n  @import \"config_wrapper\";\
             \n  @import \"midstream\";\
             \n}\n"),
                    "a b {\
         \n  c: configured;\
         \n}\n"
                );
            }
        }
        mod local {
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("local")
            }

            #[test]
            #[ignore] // wrong result
            fn through_forward() {
                let runner = runner().with_cwd("through_forward");
                assert_eq!(
                    runner.ok("// Regression test for sass/dart-sass#2641\
             \na {\
             \n  $a: shadowed;\
             \n  @import \"config_wrapper\";\
             \n  @import \"midstream\";\
             \n}\n"),
                    "a b {\
         \n  c: configured;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    #[ignore] // wrong result
    fn through_forward() {
        let runner = runner().with_cwd("through_forward");
        assert_eq!(
            runner.ok("// Regression test for sass/dart-sass#2641\
             \n$a: shadowed;\
             \n@import \"config_wrapper\";\
             \n@import \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn through_forward() {
    let runner = runner().with_cwd("through_forward");
    assert_eq!(
        runner.ok("// Regression test for sass/dart-sass#2641\
             \n@import \"config_wrapper\";\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
