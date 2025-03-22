//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/precedence.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("precedence")
        .mock_file("nested/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("nested/_upstream.scss", "$a: in-upstream;\n")
        .mock_file("top_level/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("top_level/_upstream.scss", "$a: in-upstream;\n")
}

#[test]
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.ok("b {\
             \n  $a: in-input;\n\
             \n  @import \"midstream\";\n\
             \n  c: $a;\
             \n}\n"),
        "b {\
         \n  c: in-upstream;\
         \n}\n"
    );
}
#[test]
fn top_level() {
    let runner = runner().with_cwd("top_level");
    assert_eq!(
        runner.ok("$a: in-input;\n\
             \n@import \"midstream\";\n\
             \nb {c: $a}\n"),
        "b {\
         \n  c: in-upstream;\
         \n}\n"
    );
}
