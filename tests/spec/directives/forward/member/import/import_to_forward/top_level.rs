//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/top_level.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("top_level")
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("mixin/_upstream.scss", "@mixin a() {b {c: d}}\n")
        .mock_file(
            "post_facto/with_use/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "post_facto/with_use/_other.scss",
            "@use \"sass:math\";\n\n@mixin a {b {c: $d}}\n",
        )
        .mock_file("post_facto/with_use/_upstream.scss", "$d: e;\n")
        .mock_file(
            "post_facto/without_use/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "post_facto/without_use/_other.scss",
            "@mixin a {b {c: $d}}\n",
        )
        .mock_file("post_facto/without_use/_upstream.scss", "$d: e;\n")
        .mock_file(
            "variable_assignment/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file("variable_use/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("variable_use/_upstream.scss", "$c: d;\n")
}

#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \n@include a;\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
mod post_facto {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("post_facto")
    }

    #[test]
    fn with_use() {
        let runner = runner().with_cwd("with_use");
        assert_eq!(
            runner.ok("@import \"other\";\
             \n@import \"midstream\";\n\
             \n@include a;\n"),
            "b {\
         \n  c: e;\
         \n}\n"
        );
    }
    #[test]
    fn without_use() {
        let runner = runner().with_cwd("without_use");
        assert_eq!(
            runner.ok("@import \"other\";\
             \n@import \"midstream\";\n\
             \n@include a;\n"),
            "b {\
         \n  c: e;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn variable_assignment() {
    let runner = runner().with_cwd("variable_assignment");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \n$a: new value;\n\
             \nb {c: get-a()}\n"),
        "b {\
         \n  c: new value;\
         \n}\n"
    );
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \na {b: $c}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
