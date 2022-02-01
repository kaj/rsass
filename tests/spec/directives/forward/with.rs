//! Tests auto-converted from "sass-spec/spec/directives/forward/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("core_module/indirect/forward/_midstream.scss", "// Regression test for sass/dart-sass#838.\n@forward \"upstream\" with ($c: e);\n")
        .mock_file("core_module/indirect/forward/_upstream.scss", "@forward \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("core_module/indirect/use/_midstream.scss", "// Regression test for sass/dart-sass#838.\n@forward \"upstream\" with ($c: e);\n")
        .mock_file("core_module/indirect/use/_upstream.scss", "@use \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("dash_insensitive/_midstream.scss", "@forward \"upstream\" with ($a_b: configured);\n")
        .mock_file("dash_insensitive/_upstream.scss", "$a-b: original !default;\nb {c: $a-b}\n")
        .mock_file("default/_midstream.scss", "@forward \"upstream\" with ($a: configured !default);\n")
        .mock_file("default/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("doesnt_run_default/_midstream.scss", "@forward \"upstream\" with ($a: configured);\n")
        .mock_file("doesnt_run_default/_upstream.scss", "// This will throw an error if it's evaluated, but it shouldn't be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
        .mock_file("facade_contains_multiple_configured_forwards/_left.scss", "$a: original a !default;\na {a: $a}\n")
        .mock_file("facade_contains_multiple_configured_forwards/_midstream.scss", "@forward \"left\" with ($a: configured a !default);\n@forward \"right\" with ($b: configured b !default);\n\n$c: original c !default;\nc {c: $c}\n")
        .mock_file("facade_contains_multiple_configured_forwards/_right.scss", "$b: original b !default;\nb {b: $b}\n")
        .mock_file("from_variable/_midstream.scss", "$a: configured;\n@forward \"upstream\" with ($a: $a);\n")
        .mock_file("from_variable/_upstream.scss", "$a: original a !default;\nb {c: $a}\n")
        .mock_file("multi_load/forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("multi_load/forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("multi_load/transitive/_downstream.scss", "// Regression test for sass/dart-sass#854.\n@forward \"midstream1\" as m1-* with ($a: overridden 1);\n@forward \"midstream2\" as m2-* with ($a: overridden 2);\n")
        .mock_file("multi_load/transitive/_midstream1.scss", "@use \"upstream\";\n$a: default 1 !default;\n")
        .mock_file("multi_load/transitive/_midstream2.scss", "@use \"upstream\";\n$a: default 2 !default;\n")
        .mock_file("multi_load/transitive/_upstream.scss", "c {d: e}\n")
        .mock_file("multi_load/use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("multi_load/use/_upstream.scss", "$a: original !default;\n")
        .mock_file("multiple/default/_midstream.scss", "@forward \"upstream\" with (\n  $a: configured a !default,\n  $b: configured b !default,\n  $c: configured c !default\n);\n")
        .mock_file("multiple/default/_upstream.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
        .mock_file("multiple/non_default/_midstream.scss", "@forward \"upstream\" with (\n  $a: configured a,\n  $b: configured b,\n  $c: configured c\n);\n")
        .mock_file("multiple/non_default/_upstream.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
        .mock_file("null/_midstream.scss", "@forward \"upstream\" with ($a: null);\n")
        .mock_file("null/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("single/_midstream.scss", "@forward \"upstream\" with ($a: configured);\n")
        .mock_file("single/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("some_unconfigured/_midstream.scss", "@forward \"upstream\" with ($a: configured a);\n")
        .mock_file("some_unconfigured/_upstream.scss", "$a: original a !default;\n$b: original b !default;\n\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/and_use/_definition.scss", "$c: d !default;\n")
        .mock_file("through_forward/and_use/_downstream.scss", "// Regression test for sass/sass#2744.\n@use \"midstream\" with ($c: e);\n\na {b: midstream.$c}\n")
        .mock_file("through_forward/and_use/_midstream.scss", "@forward \"definition\";\n@forward \"user\";\n")
        .mock_file("through_forward/and_use/_user.scss", "@use \"definition\";\n")
        .mock_file("through_forward/as/_downstream.scss", "@forward \"midstream\" with ($b-a: configured);\n")
        .mock_file("through_forward/as/_midstream.scss", "@forward \"upstream\" as b-*;\n")
        .mock_file("through_forward/as/_upstream.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("through_forward/bare/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("through_forward/bare/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("through_forward/bare/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/hide/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("through_forward/hide/_midstream.scss", "@forward \"upstream\" hide $b;\n")
        .mock_file("through_forward/hide/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/show/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("through_forward/show/_midstream.scss", "@forward \"upstream\" show $a;\n")
        .mock_file("through_forward/show/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/default/_downstream.scss", "@forward \"midstream\" with ($a: from downstream);\n")
        .mock_file("through_forward/with/default/_midstream.scss", "@forward \"upstream\" with ($a: from midstream !default);\n")
        .mock_file("through_forward/with/default/_upstream.scss", "$a: from upstream !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/null/_downstream.scss", "@forward \"midstream\" with ($a: null);\n")
        .mock_file("through_forward/with/null/_midstream.scss", "@forward \"upstream\" with ($a: from midstream !default);\n")
        .mock_file("through_forward/with/null/_upstream.scss", "$a: from upstream !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/unconfigured/_downstream.scss", "@forward \"midstream\" with ($a: from downstream);\n")
        .mock_file("through_forward/with/unconfigured/_midstream.scss", "@forward \"upstream\" with ($b: from midstream !default);\n")
        .mock_file("through_forward/with/unconfigured/_upstream.scss", "$a: from upstream !default;\n$b: from upstream !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/with_unrelated_config/_downstream.scss", "@forward \"midstream\" with ($from-midstream: configured);\n")
        .mock_file("through_forward/with_unrelated_config/_midstream.scss", "@forward \"upstream\";\n\n$from-midstream: original !default;\n\na {from-midstream: $from-midstream}\n")
        .mock_file("through_forward/with_unrelated_config/_upstream.scss", "$from-upstream: original !default;\nb {from-upstream: $from-upstream}\n")
        .mock_file("through_import/direct/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("through_import/direct/_midstream.scss", "@import \"upstream\";\n")
        .mock_file("through_import/direct/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_import/transitive/_forwarded.scss", "@import \"imported_downstream\";\n")
        .mock_file("through_import/transitive/_imported_downstream.scss", "@import \"imported_upstream\";\n")
        .mock_file("through_import/transitive/_imported_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_import/transitive/_used.scss", "@forward \"forwarded\" with ($a: configured);\n")
        .mock_file("trailing_comma/default/_midstream.scss", "@forward \"upstream\" with ($a: configured !default,);\n")
        .mock_file("trailing_comma/default/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("trailing_comma/single/_midstream.scss", "@forward \"upstream\" with ($a: configured,);\n")
        .mock_file("trailing_comma/single/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("used_in_input/_midstream.scss", "@forward \"upstream\" with ($a: configured);\n")
        .mock_file("used_in_input/_upstream.scss", "$a: original !default;\n")
        .mock_file("variable_exists/_midstream.scss", "@forward \"upstream\" with ($a: configured);\n")
        .mock_file("variable_exists/_upstream.scss", "$before-declaration: variable-exists(a);\n$a: original !default;\nb {\n  before-declaration: $before-declaration;\n  after-declaration: variable-exists(a);\n}\n")
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
                runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
        #[test]
        fn test_use() {
            let runner = runner().with_cwd("use");
            assert_eq!(
                runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
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
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn default() {
    let runner = runner().with_cwd("default");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn doesnt_run_default() {
    let runner = runner().with_cwd("doesnt_run_default");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn facade_contains_multiple_configured_forwards() {
    let runner =
        runner().with_cwd("facade_contains_multiple_configured_forwards");
    assert_eq!(
        runner.ok("// Regression test for sass/dart-sass/#1343.\
             \n@use \"midstream\" with (\
             \n  $a: twice-configured a,\
             \n  $b: twice-configured b,\
             \n  $c: configured c,\
             \n);\n"),
        "a {\
         \n  a: twice-configured a;\
         \n}\
         \nb {\
         \n  b: twice-configured b;\
         \n}\
         \nc {\
         \n  c: configured c;\
         \n}\n"
    );
}
#[test]
fn from_variable() {
    let runner = runner().with_cwd("from_variable");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
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
            runner.ok("@forward \"upstream\" with ($a: configured);\
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
            runner.ok("@use \"downstream\";\n\
             \nb {\
             \n  midstream1: downstream.$m1-a;\
             \n  midstream2: downstream.$m2-a;\
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
            runner.ok("@forward \"upstream\" with ($a: configured);\
             \n@use \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
mod multiple {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("multiple")
    }

    #[test]
    fn default() {
        let runner = runner().with_cwd("default");
        assert_eq!(
            runner.ok("@use \"midstream\";\n"),
            "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
        );
    }
    #[test]
    fn non_default() {
        let runner = runner().with_cwd("non_default");
        assert_eq!(
            runner.ok("@use \"midstream\";\n"),
            "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
        );
    }
}
#[test]
fn null() {
    let runner = runner().with_cwd("null");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: original;\
         \n}\n"
    );
}
#[test]
fn single() {
    let runner = runner().with_cwd("single");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn some_unconfigured() {
    let runner = runner().with_cwd("some_unconfigured");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
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
            runner.ok("@use \"downstream\";\n"),
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
            runner.ok("@use \"downstream\";\n"),
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
            runner.ok("@use \"downstream\";\n"),
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
            runner.ok("@use \"downstream\";\n"),
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
            runner.ok("@use \"downstream\";\n"),
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

        #[test]
        fn default() {
            let runner = runner().with_cwd("default");
            assert_eq!(
                runner.ok("@use \"downstream\";\n"),
                "b {\
         \n  c: from downstream;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn null() {
            let runner = runner().with_cwd("null");
            assert_eq!(
                runner.ok("@use \"downstream\";\n"),
                "b {\
         \n  c: from midstream;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unconfigured() {
            let runner = runner().with_cwd("unconfigured");
            assert_eq!(
                runner.ok("@use \"downstream\";\n"),
                "c {\
         \n  a: from downstream;\
         \n  b: from midstream;\
         \n}\n"
            );
        }
    }
    #[test]
    fn with_unrelated_config() {
        let runner = runner().with_cwd("with_unrelated_config");
        assert_eq!(
            runner.ok("@use \"downstream\";\n"),
            "b {\
         \n  from-upstream: original;\
         \n}\
         \na {\
         \n  from-midstream: configured;\
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
            runner.ok("@use \"downstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    fn transitive() {
        let runner = runner().with_cwd("transitive");
        assert_eq!(
            runner.ok("@use \"used\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
mod trailing_comma {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("trailing_comma")
    }

    #[test]
    fn default() {
        let runner = runner().with_cwd("default");
        assert_eq!(
            runner.ok("@use \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        let runner = runner().with_cwd("single");
        assert_eq!(
            runner.ok("@use \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
fn used_in_input() {
    let runner = runner().with_cwd("used_in_input");
    assert_eq!(
        runner.ok("@use \"midstream\";\
             \nb {c: midstream.$a}\n"),
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
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  before-declaration: false;\
         \n  after-declaration: true;\
         \n}\n"
    );
}
