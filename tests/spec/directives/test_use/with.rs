//! Tests auto-converted from "sass-spec/spec/directives/use/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("core_module/indirect/forward/_other.scss", "@forward \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("core_module/indirect/use/_other.scss", "@use \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("dash_insensitive/_other.scss", "$a-b: original !default;\nb {c: $a-b}\n")
        .mock_file("doesnt_run_default/_other.scss", "// This will throw an error if it's evaluated, but it shouldn't be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
        .mock_file("from_variable/_other.scss", "$a: original a !default;\nb {c: $a}\n")
        .mock_file("multi_load/forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("multi_load/forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("multi_load/transitive/_midstream1.scss", "@use \"upstream\";\n$a: default 1 !default;\n")
        .mock_file("multi_load/transitive/_midstream2.scss", "@use \"upstream\";\n$a: default 2 !default;\n")
        .mock_file("multi_load/transitive/_upstream.scss", "c {d: e}\n")
        .mock_file("multi_load/use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("multi_load/use/_upstream.scss", "$a: original !default;\n")
        .mock_file("multiple/_other.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
        .mock_file("null/_other.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("single/_other.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("some_unconfigured/_other.scss", "$a: original a !default;\n$b: original b !default;\n\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/and_use/_definition.scss", "$c: d !default;\n")
        .mock_file("through_forward/and_use/_forwarder.scss", "@forward \"definition\";\n@forward \"user\";\n")
        .mock_file("through_forward/and_use/_user.scss", "@use \"definition\";\n")
        .mock_file("through_forward/as/_forwarded.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("through_forward/as/_used.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("through_forward/bare/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/bare/_used.scss", "@forward \"forwarded\";\n")
        .mock_file("through_forward/hide/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/hide/_used.scss", "@forward \"forwarded\" hide $b;\n")
        .mock_file("through_forward/show/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/show/_used.scss", "@forward \"forwarded\" show $a;\n")
        .mock_file("through_forward/transitive/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("through_forward/transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/transitive/_used.scss", "@forward \"midstream\";\n")
        .mock_file("through_forward/with/and_variable/after/_forwarded.scss", "$b: from forwarded !default;\nin-forwarded {d: $b}\n")
        .mock_file("through_forward/with/and_variable/after/_used.scss", "@forward \"forwarded\" with ($b: from used !default);\n$a: from used !default;\nin-used {c: $a}\n")
        .mock_file("through_forward/with/and_variable/before/_forwarded.scss", "$b: from forwarded !default;\nin-forwarded {d: $b}\n")
        .mock_file("through_forward/with/and_variable/before/_used.scss", "$a: from used !default;\n@forward \"forwarded\" with ($b: from used !default);\nin-used {c: $a}\n")
        .mock_file("through_forward/with/default/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/default/_used.scss", "@forward \"forwarded\" with ($a: from used !default);\n")
        .mock_file("through_forward/with/multiple/_left.scss", "$a: from left !default;\nin-left {c: $a}\n")
        .mock_file("through_forward/with/multiple/_right.scss", "$b: from left !default;\nin-right {d: $b}\n")
        .mock_file("through_forward/with/multiple/_used.scss", "@forward \"left\" with ($a: from used !default);\n@forward \"right\" with ($b: from used !default);\n")
        .mock_file("through_forward/with/null/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/null/_used.scss", "@forward \"forwarded\" with ($a: from used !default);\n")
        .mock_file("through_forward/with/unconfigured/_forwarded.scss", "$a: from forwarded !default;\n$b: from forwarded !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/with/unconfigured/_used.scss", "@forward \"forwarded\" with ($b: from used);\n")
        .mock_file("through_forward/with_unrelated_config/_forwarded.scss", "$from-forwarded: original !default;\nb {from-forwarded: $from-forwarded}\n")
        .mock_file("through_forward/with_unrelated_config/_used.scss", "@forward \"forwarded\";\n\n$from-used: original !default;\n\na {from-used: $from-used}\n")
        .mock_file("through_import/direct/_imported.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_import/direct/_used.scss", "@import \"imported\";\n")
        .mock_file("through_import/transitive/_midstream.scss", "@import \"upstream\";\n")
        .mock_file("through_import/transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_import/transitive/_used.scss", "@import \"midstream\";\n")
        .mock_file("trailing_comma/_other.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("used_in_input/_other.scss", "$a: original !default;\n")
        .mock_file("variable_exists/_other.scss", "$before-declaration: variable-exists(a);\n$a: original !default;\nb {\n  before-declaration: $before-declaration;\n  after-declaration: variable-exists(a);\n}\n")
}

mod core_module {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("core_module")
    }

    mod indirect {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("indirect")
        }

        #[test]
        fn forward() {
            let runner = runner().with_cwd("forward");
            assert_eq!(
                runner.ok("// Regression test for sass/dart-sass#838.\
             \n@use \"other\" with ($c: e);\n\
             \na {b: other.$c}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
        #[test]
        fn test_use() {
            let runner = runner().with_cwd("use");
            assert_eq!(
                runner.ok("// Regression test for sass/dart-sass#838.\
             \n@use \"other\" with ($c: e);\n\
             \na {b: other.$c}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
    }
}
#[test]
fn dash_insensitive() {
    let runner = runner().with_cwd("dash_insensitive");
    assert_eq!(
        runner.ok("@use \"other\" with ($a_b: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn doesnt_run_default() {
    let runner = runner().with_cwd("doesnt_run_default");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn from_variable() {
    let runner = runner().with_cwd("from_variable");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@use \"other\" with ($a: $a);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
mod multi_load {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("multi_load")
    }

    #[test]
    fn forward() {
        let runner = runner().with_cwd("forward");
        assert_eq!(
            runner.ok("@use \"upstream\" with ($a: configured);\
             \n@use \"midstream\";\
             \nb {c: midstream.$a}\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    fn transitive() {
        let runner = runner().with_cwd("transitive");
        assert_eq!(
            runner.ok("// Regression test for sass/dart-sass#854.\
             \n@use \"midstream1\" with ($a: overridden 1);\
             \n@use \"midstream2\" with ($a: overridden 2);\n\
             \nb {\
             \n  midstream1: midstream1.$a;\
             \n  midstream2: midstream2.$a;\
             \n}\n"),
            "c {\
         \n  d: e;\
         \n}\
         \nb {\
         \n  midstream1: overridden 1;\
         \n  midstream2: overridden 2;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        let runner = runner().with_cwd("use");
        assert_eq!(
            runner.ok("@use \"upstream\" with ($a: configured);\
             \n@use \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
fn multiple() {
    let runner = runner().with_cwd("multiple");
    assert_eq!(
        runner.ok("@use \"other\" with (\
             \n  $a: configured a,\
             \n  $b: configured b,\
             \n  $c: configured c\
             \n);\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
#[test]
fn null() {
    let runner = runner().with_cwd("null");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: null);\n"),
        "b {\
         \n  c: original;\
         \n}\n"
    );
}
#[test]
fn single() {
    let runner = runner().with_cwd("single");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn some_unconfigured() {
    let runner = runner().with_cwd("some_unconfigured");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured a);\n"),
        "c {\
         \n  a: configured a;\
         \n  b: original b;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    fn and_use() {
        let runner = runner().with_cwd("and_use");
        assert_eq!(
            runner.ok("// Regression test for sass/sass#2744.\
             \n@use \"forwarder\" with ($c: e);\n\
             \na {b: forwarder.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
            runner.ok("@use \"used\" with ($b-a: configured);\n"),
            "c {\
         \n  d: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn transitive() {
        let runner = runner().with_cwd("transitive");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    mod with {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("with")
        }

        mod and_variable {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("and_variable")
            }

            #[test]
            fn after() {
                let runner = runner().with_cwd("after");
                assert_eq!(
                    runner.ok("// Regression test for sass/dart-sass#1460\
             \n@use \"used\" with ($a: from input, $b: from input);\n"),
                    "in-forwarded {\
         \n  d: from input;\
         \n}\
         \nin-used {\
         \n  c: from input;\
         \n}\n"
                );
            }
            #[test]
            fn before() {
                let runner = runner().with_cwd("before");
                assert_eq!(
        runner.ok(
            "@use \"used\" with ($a: from input, $b: from input);\n"
        ),
        "in-forwarded {\
         \n  d: from input;\
         \n}\
         \nin-used {\
         \n  c: from input;\
         \n}\n"
    );
            }
        }
        #[test]
        fn default() {
            let runner = runner().with_cwd("default");
            assert_eq!(
                runner.ok("@use \"used\" with ($a: from input);\n"),
                "b {\
         \n  c: from input;\
         \n}\n"
            );
        }
        #[test]
        fn multiple() {
            let runner = runner().with_cwd("multiple");
            assert_eq!(
                runner.ok(
                    "@use \"used\" with ($a: from input, $b: from input);\n"
                ),
                "in-left {\
         \n  c: from input;\
         \n}\
         \nin-right {\
         \n  d: from input;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn null() {
            let runner = runner().with_cwd("null");
            assert_eq!(
                runner.ok("@use \"used\" with ($a: null);\n"),
                "b {\
         \n  c: from used;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unconfigured() {
            let runner = runner().with_cwd("unconfigured");
            assert_eq!(
                runner.ok("@use \"used\" with ($a: from input);\n"),
                "c {\
         \n  a: from input;\
         \n  b: from used;\
         \n}\n"
            );
        }
    }
    #[test]
    fn with_unrelated_config() {
        let runner = runner().with_cwd("with_unrelated_config");
        assert_eq!(
            runner.ok("@use \"used\" with ($from-used: configured);\n"),
            "b {\
         \n  from-forwarded: original;\
         \n}\
         \na {\
         \n  from-used: configured;\
         \n}\n"
        );
    }
}
mod through_import {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_import")
    }

    #[test]
    fn direct() {
        let runner = runner().with_cwd("direct");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    fn transitive() {
        let runner = runner().with_cwd("transitive");
        assert_eq!(
            runner.ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
fn trailing_comma() {
    let runner = runner().with_cwd("trailing_comma");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured,);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn used_in_input() {
    let runner = runner().with_cwd("used_in_input");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured);\
             \nb {c: other.$a}\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn variable_exists() {
    let runner = runner().with_cwd("variable_exists");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  before-declaration: false;\
         \n  after-declaration: true;\
         \n}\n"
    );
}
