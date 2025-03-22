//! Tests auto-converted from "sass-spec/spec/directives/use/member/use_to_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("use_to_import")
        .mock_file("function/midstream.scss", "@import \"upstream\";\n")
        .mock_file(
            "function/upstream.scss",
            "@function member() {@return value}\n",
        )
        .mock_file("mixin/midstream.scss", "@import \"upstream\";\n")
        .mock_file("mixin/upstream.scss", "@mixin member() {a {b: c}}\n")
        .mock_file(
            "variable_assignment/midstream.scss",
            "@import \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/upstream.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file("variable_use/midstream.scss", "@import \"upstream\";\n")
        .mock_file("variable_use/upstream.scss", "$member: value;\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.member()}\n"),
        "a {\
         \n  b: value;\
         \n}\n"
    );
}
#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \n@include midstream.member;\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn variable_assignment() {
    let runner = runner().with_cwd("variable_assignment");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nmidstream.$member: new value;\n\
             \na {b: midstream.get-member()}\n"),
        "a {\
         \n  b: new value;\
         \n}\n"
    );
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$member}\n"),
        "a {\
         \n  b: value;\
         \n}\n"
    );
}
