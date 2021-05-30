//! Tests auto-converted from "sass-spec/spec/directives/forward/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "forward_into_import/_forwarded.scss",
            "@import \"imported\";\n",
        )
        .mock_file(
            "forward_into_import/_imported.scss",
            "in-imported {a: b}\n",
        )
        .mock_file(
            "forward_into_import/_midstream.scss",
            "@forward \"forwarded\";\n",
        )
        .mock_file("forward_into_use/_forwarded.scss", "@use \"used\";\n")
        .mock_file(
            "forward_into_use/_midstream.scss",
            "@forward \"forwarded\";\n",
        )
        .mock_file("forward_into_use/_used.scss", "in-used {a: b}\n")
        .mock_file("upstream/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("upstream/_upstream.scss", "in-upstream {a: b}\n")
}

#[test]
#[ignore] // wrong result
fn forward_into_import() {
    let runner = runner().with_cwd("forward_into_import");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-imported}\n"),
        "in-imported, in-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn forward_into_use() {
    let runner = runner().with_cwd("forward_into_use");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-used}\n"),
        "in-used, in-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn upstream() {
    let runner = runner().with_cwd("upstream");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-upstream}\n"),
        "in-upstream, in-input {\
         \n  a: b;\
         \n}\n"
    );
}
