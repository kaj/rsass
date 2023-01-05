//! Tests auto-converted from "sass-spec/spec/directives/forward/with/multi_load.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multi_load")
        .mock_file("forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("transitive/_downstream.scss", "// Regression test for sass/dart-sass#854.\n@forward \"midstream1\" as m1-* with ($a: overridden 1);\n@forward \"midstream2\" as m2-* with ($a: overridden 2);\n")
        .mock_file("transitive/_midstream1.scss", "@use \"upstream\";\n$a: default 1 !default;\n")
        .mock_file("transitive/_midstream2.scss", "@use \"upstream\";\n$a: default 2 !default;\n")
        .mock_file("transitive/_upstream.scss", "c {d: e}\n")
        .mock_file("use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("use/_upstream.scss", "$a: original !default;\n")
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
