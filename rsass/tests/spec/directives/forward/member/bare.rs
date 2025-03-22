//! Tests auto-converted from "sass-spec/spec/directives/forward/member/bare.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("bare")
        .mock_file("function/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("function/_upstream.scss", "@function c() {@return d}\n")
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("mixin/_upstream.scss", "@mixin a() {b {c: d}}\n")
        .mock_file(
            "no_conflict/function/_midstream.scss",
            "@forward \"upstream\";\n@forward \"upstream\";\n",
        )
        .mock_file(
            "no_conflict/function/_upstream.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "no_conflict/mixin/_midstream.scss",
            "@forward \"upstream\";\n@forward \"upstream\";\n",
        )
        .mock_file("no_conflict/mixin/_upstream.scss", "@mixin b {c: d}\n")
        .mock_file(
            "no_conflict/variable/_midstream.scss",
            "@forward \"upstream\";\n@forward \"upstream\";\n",
        )
        .mock_file("no_conflict/variable/_upstream.scss", "$c: d;\n")
        .mock_file(
            "variable_assignment/nested/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/nested/_upstream.scss",
            "$b: old value;\n\n@function get-b() {@return $b}\n",
        )
        .mock_file(
            "variable_assignment/top_level/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "variable_assignment/top_level/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file("variable_use/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("variable_use/_upstream.scss", "$c: d;\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.c()}\n"),
        "a {\
         \n  b: d;\
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
         \n  c: d;\
         \n}\n"
    );
}
mod no_conflict {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("no_conflict")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.c()}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {@include midstream.b}\n"),
            "a {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
}
mod variable_assignment {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("variable_assignment")
    }

    #[test]
    #[ignore] // unexepected error
    fn nested() {
        let runner = runner().with_cwd("nested");
        assert_eq!(
        runner.ok(
            "@use \"midstream\";\n\
             \na {\
             \n  // Namespaced assignments always assign to the other module\'s variable, even\
             \n  // if they\'re nested in a block scope.\
             \n  midstream.$b: new value;\n\
             \n  c: midstream.get-b();\
             \n}\n"
        ),
        "a {\
         \n  c: new value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn top_level() {
        let runner = runner().with_cwd("top_level");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nmidstream.$a: new value;\n\
             \nb {c: midstream.get-a()};\n"),
            "b {\
         \n  c: new value;\
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
         \n  b: d;\
         \n}\n"
    );
}
