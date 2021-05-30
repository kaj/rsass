//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/use_to.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("mixin/_forwarded.scss", "@mixin a() {b {c: d}}\n")
        .mock_file("mixin/_imported.scss", "@forward \"forwarded\";\n")
        .mock_file("mixin/_used.scss", "@import \"imported\";\n")
        .mock_file(
            "variable_assignment/_forwarded.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file(
            "variable_assignment/_imported.scss",
            "@forward \"forwarded\";\n",
        )
        .mock_file(
            "variable_assignment/_used.scss",
            "@import \"imported\";\n",
        )
        .mock_file("variable_use/_forwarded.scss", "$c: d;\n")
        .mock_file("variable_use/_imported.scss", "@forward \"forwarded\";\n")
        .mock_file("variable_use/_used.scss", "@import \"imported\";\n")
}

#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@use \"used\";\n\
             \n@include used.a;\n"),
        "b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn variable_assignment() {
    let runner = runner().with_cwd("variable_assignment");
    assert_eq!(
        runner.ok("@use \"used\";\n\
             \nused.$a: new value;\n\
             \nb {c: used.get-a()}\n"),
        "b {\
         \n  c: new value;\
         \n}\n"
    );
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@use \"used\";\n\
             \na {b: used.$c}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
