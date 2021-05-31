//! Tests auto-converted from "sass-spec/spec/directives/forward/member/shadowed.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("function/_midstream.scss", "@forward \"upstream\";\n\n@function c() {@return midstream}\n")
        .mock_file("function/_upstream.scss", "@function c() {@return upstream}\n")
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\";\n\n@mixin a() {b {c: midstream}}\n")
        .mock_file("mixin/_upstream.scss", "@mixin a() {b {c: upstream}}\n")
        .mock_file("variable_assignment/top_level/_midstream.scss", "@forward \"upstream\";\n\n$a: midstream value;\n\n@function get-midstream-a() {@return $a}\n")
        .mock_file("variable_assignment/top_level/_upstream.scss", "$a: upstream value;\n\n@function get-upstream-a() {@return $a}\n")
        .mock_file("variable_use/_midstream.scss", "@forward \"upstream\";\n\n$c: midstream;\n")
        .mock_file("variable_use/_upstream.scss", "$c: upstream;\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.c()}\n"),
        "a {\
         \n  b: midstream;\
         \n}\n"
    );
}
#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \n@include midstream.a;\n"),
        "b {\
         \n  c: midstream;\
         \n}\n"
    );
}
mod variable_assignment {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("variable_assignment")
    }

    #[test]
    #[ignore] // unexepected error
    fn top_level() {
        let runner = runner().with_cwd("top_level");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nmidstream.$a: new value;\n\
             \nb {\
             \n  midstream: midstream.get-midstream-a();\
             \n  upstream: midstream.get-upstream-a();\
             \n};\n"),
            "b {\
         \n  midstream: midstream value;\
         \n  upstream: new value;\
         \n}\n"
        );
    }
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
        "a {\
         \n  b: midstream;\
         \n}\n"
    );
}
