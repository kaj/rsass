//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("nested")
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("mixin/_upstream.scss", "@mixin b() {c: d}\n")
        .mock_file(
            "variable_assignment/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/_upstream.scss",
            "$b: old value;\n\n@function get-b() {@return $b}\n",
        )
        .mock_file("variable_use/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("variable_use/_upstream.scss", "$c: d;\n")
}

#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"midstream\";\n\
             \n  @include b;\
             \n}\n"),
        "a {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn variable_assignment() {
    let runner = runner().with_cwd("variable_assignment");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"midstream\";\n\
             \n  $b: new value;\
             \n  c: get-b();\
             \n}\n"),
        "a {\
         \n  c: new value;\
         \n}\n"
    );
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("a {\
             \n  @import \"midstream\";\n\
             \n  b: $c;\
             \n}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
