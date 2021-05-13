//! Tests auto-converted from "sass-spec/spec/directives/use/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("core_module/indirect/forward/_other.scss", "@forward \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("core_module/indirect/use/_other.scss", "@use \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("dash_insensitive/_other.scss", "$a-b: original !default;\nb {c: $a-b}\n")
        .mock_file("doesnt_run_default/_other.scss", "// This will throw an error if it\'s evaluated, but it shouldn\'t be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
        .mock_file("from_variable/_other.scss", "$a: original a !default;\nb {c: $a}\n")
        .mock_file("multi_load/forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("multi_load/forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("multi_load/transitive/_midstream1.scss", "@use \"upstream\";\n$a: default 1 !default;\n")
        .mock_file("multi_load/transitive/_midstream2.scss", "@use \"upstream\";\n$a: default 2 !default;\n")
        .mock_file("multi_load/transitive/_upstream.scss", "c {d: e}\n")
        .mock_file("multi_load/use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("multi_load/use/_upstream.scss", "$a: original !default;\n")
        .mock_file("multiple/_other.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
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
        .mock_file("through_forward/with/default/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/default/_used.scss", "@forward \"forwarded\" with ($a: from used !default);\n")
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
    use super::runner;
    mod indirect {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // unexepected error
        fn forward() {
            assert_eq!(
                runner().ok("// Regression test for sass/dart-sass#838.\
             \n@use \"other\" with ($c: e);\n\
             \na {b: other.$c}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn test_use() {
            assert_eq!(
                runner().ok("// Regression test for sass/dart-sass#838.\
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
#[ignore] // unexepected error
fn dash_insensitive() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a_b: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn doesnt_run_default() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn from_variable() {
    assert_eq!(
        runner().ok("$a: configured;\
             \n@use \"other\" with ($a: $a);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
mod multi_load {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn forward() {
        assert_eq!(
            runner().ok("@use \"upstream\" with ($a: configured);\
             \n@use \"midstream\";\
             \nb {c: midstream.$a}\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#854.\
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
    #[ignore] // unexepected error
    fn test_use() {
        assert_eq!(
            runner().ok("@use \"upstream\" with ($a: configured);\
             \n@use \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        runner().ok("@use \"other\" with (\
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
#[ignore] // unexepected error
fn single() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn some_unconfigured() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured a);\n"),
        "c {\
         \n  a: configured a;\
         \n  b: original b;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn and_use() {
        assert_eq!(
            runner().ok("// Regression test for sass/sass#2744.\
             \n@use \"forwarder\" with ($c: e);\n\
             \na {b: forwarder.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            runner().ok("@use \"used\" with ($b-a: configured);\n"),
            "c {\
         \n  d: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    mod with {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // unexepected error
        fn default() {
            assert_eq!(
                runner().ok("@use \"used\" with ($a: from input);\n"),
                "b {\
         \n  c: from input;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn null() {
            assert_eq!(
                runner().ok("@use \"used\" with ($a: null);\n"),
                "b {\
         \n  c: from used;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn unconfigured() {
            assert_eq!(
                runner().ok("@use \"used\" with ($a: from input);\n"),
                "c {\
         \n  a: from input;\
         \n  b: from used;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn with_unrelated_config() {
        assert_eq!(
            runner().ok("@use \"used\" with ($from-used: configured);\n"),
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
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn direct() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            runner().ok("@use \"used\" with ($a: configured);\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn trailing_comma() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured,);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn used_in_input() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\
             \nb {c: other.$a}\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn variable_exists() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\n"),
        "b {\
         \n  before-declaration: false;\
         \n  after-declaration: true;\
         \n}\n"
    );
}
