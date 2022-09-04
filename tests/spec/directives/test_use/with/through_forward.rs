//! Tests auto-converted from "sass-spec/spec/directives/use/with/through_forward.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_forward")
        .mock_file("and_use/_definition.scss", "$c: d !default;\n")
        .mock_file("and_use/_forwarder.scss", "@forward \"definition\";\n@forward \"user\";\n")
        .mock_file("and_use/_user.scss", "@use \"definition\";\n")
        .mock_file("as/_forwarded.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("as/_used.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("bare/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("bare/_used.scss", "@forward \"forwarded\";\n")
        .mock_file("hide/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("hide/_used.scss", "@forward \"forwarded\" hide $b;\n")
        .mock_file("show/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("show/_used.scss", "@forward \"forwarded\" show $a;\n")
        .mock_file("transitive/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("transitive/_used.scss", "@forward \"midstream\";\n")
        .mock_file("with/and_variable/after/_forwarded.scss", "$b: from forwarded !default;\nin-forwarded {d: $b}\n")
        .mock_file("with/and_variable/after/_used.scss", "@forward \"forwarded\" with ($b: from used !default);\n$a: from used !default;\nin-used {c: $a}\n")
        .mock_file("with/and_variable/before/_forwarded.scss", "$b: from forwarded !default;\nin-forwarded {d: $b}\n")
        .mock_file("with/and_variable/before/_used.scss", "$a: from used !default;\n@forward \"forwarded\" with ($b: from used !default);\nin-used {c: $a}\n")
        .mock_file("with/default/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("with/default/_used.scss", "@forward \"forwarded\" with ($a: from used !default);\n")
        .mock_file("with/multiple/_left.scss", "$a: from left !default;\nin-left {c: $a}\n")
        .mock_file("with/multiple/_right.scss", "$b: from left !default;\nin-right {d: $b}\n")
        .mock_file("with/multiple/_used.scss", "@forward \"left\" with ($a: from used !default);\n@forward \"right\" with ($b: from used !default);\n")
        .mock_file("with/null/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("with/null/_used.scss", "@forward \"forwarded\" with ($a: from used !default);\n")
        .mock_file("with/unconfigured/_forwarded.scss", "$a: from forwarded !default;\n$b: from forwarded !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("with/unconfigured/_used.scss", "@forward \"forwarded\" with ($b: from used);\n")
        .mock_file("with_unrelated_config/_forwarded.scss", "$from-forwarded: original !default;\nb {from-forwarded: $from-forwarded}\n")
        .mock_file("with_unrelated_config/_used.scss", "@forward \"forwarded\";\n\n$from-used: original !default;\n\na {from-used: $from-used}\n")
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
            runner
                .ok("@use \"used\" with ($a: from input, $b: from input);\n"),
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
