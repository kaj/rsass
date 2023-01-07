//! Tests auto-converted from "sass-spec/spec/directives/forward/with/through_forward.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_forward")
        .mock_file("and_use/_definition.scss", "$c: d !default;\n")
        .mock_file("and_use/_downstream.scss", "// Regression test for sass/sass#2744.\n@use \"midstream\" with ($c: e);\n\na {b: midstream.$c}\n")
        .mock_file("and_use/_midstream.scss", "@forward \"definition\";\n@forward \"user\";\n")
        .mock_file("and_use/_user.scss", "@use \"definition\";\n")
        .mock_file("as/_downstream.scss", "@forward \"midstream\" with ($b-a: configured);\n")
        .mock_file("as/_midstream.scss", "@forward \"upstream\" as b-*;\n")
        .mock_file("as/_upstream.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("bare/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("bare/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("bare/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("hide/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("hide/_midstream.scss", "@forward \"upstream\" hide $b;\n")
        .mock_file("hide/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("show/_downstream.scss", "@forward \"midstream\" with ($a: configured);\n")
        .mock_file("show/_midstream.scss", "@forward \"upstream\" show $a;\n")
        .mock_file("show/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("with/default/_downstream.scss", "@forward \"midstream\" with ($a: from downstream);\n")
        .mock_file("with/default/_midstream.scss", "@forward \"upstream\" with ($a: from midstream !default);\n")
        .mock_file("with/default/_upstream.scss", "$a: from upstream !default;\nb {c: $a}\n")
        .mock_file("with/null/_downstream.scss", "@forward \"midstream\" with ($a: null);\n")
        .mock_file("with/null/_midstream.scss", "@forward \"upstream\" with ($a: from midstream !default);\n")
        .mock_file("with/null/_upstream.scss", "$a: from upstream !default;\nb {c: $a}\n")
        .mock_file("with/unconfigured/_downstream.scss", "@forward \"midstream\" with ($a: from downstream);\n")
        .mock_file("with/unconfigured/_midstream.scss", "@forward \"upstream\" with ($b: from midstream !default);\n")
        .mock_file("with/unconfigured/_upstream.scss", "$a: from upstream !default;\n$b: from upstream !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("with_unrelated_config/_downstream.scss", "@forward \"midstream\" with ($from-midstream: configured);\n")
        .mock_file("with_unrelated_config/_midstream.scss", "@forward \"upstream\";\n\n$from-midstream: original !default;\n\na {from-midstream: $from-midstream}\n")
        .mock_file("with_unrelated_config/_upstream.scss", "$from-upstream: original !default;\nb {from-upstream: $from-upstream}\n")
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
