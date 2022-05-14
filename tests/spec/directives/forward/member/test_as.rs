//! Tests auto-converted from "sass-spec/spec/directives/forward/member/as.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("as")
        .mock_file(
            "different_separator/_midstream.scss",
            "@forward \"upstream\" as d_*;\n",
        )
        .mock_file("different_separator/_upstream.scss", "$c: e;\n")
        .mock_file(
            "function/_midstream.scss",
            "@forward \"upstream\" as d-*;\n",
        )
        .mock_file("function/_upstream.scss", "@function c() {@return e}\n")
        .mock_file(
            "hide/_midstream.scss",
            "@forward \"upstream\" as b-* hide a;\n",
        )
        .mock_file("hide/_upstream.scss", "@mixin a() {c {d: e}}\n")
        .mock_file("mixin/_midstream.scss", "@forward \"upstream\" as b-*;\n")
        .mock_file("mixin/_upstream.scss", "@mixin a() {c {d: e}}\n")
        .mock_file(
            "no_conflict/function/_midstream.scss",
            "@forward \"upstream\" as c-*;\n@forward \"upstream\" as c-*;\n",
        )
        .mock_file(
            "no_conflict/function/_upstream.scss",
            "@function d() {@return e}\n",
        )
        .mock_file(
            "no_conflict/mixin/_midstream.scss",
            "@forward \"upstream\" as b-*;\n@forward \"upstream\" as b-*;\n",
        )
        .mock_file("no_conflict/mixin/_upstream.scss", "@mixin c {d: e}\n")
        .mock_file(
            "no_conflict/variable/_midstream.scss",
            "@forward \"upstream\" as c-*;\n@forward \"upstream\" as c-*;\n",
        )
        .mock_file("no_conflict/variable/_upstream.scss", "$d: e;\n")
        .mock_file(
            "show/different_separator/_midstream.scss",
            "@forward \"upstream\" as b-* show b_a;\n",
        )
        .mock_file(
            "show/different_separator/_upstream.scss",
            "@mixin a() {c {d: e}}\n",
        )
        .mock_file(
            "show/same_separator/_midstream.scss",
            "@forward \"upstream\" as b-* show b-a;\n",
        )
        .mock_file(
            "show/same_separator/_upstream.scss",
            "@mixin a() {c {d: e}}\n",
        )
        .mock_file(
            "variable_assignment/nested/_midstream.scss",
            "@forward \"upstream\" as d-*;\n",
        )
        .mock_file(
            "variable_assignment/nested/_upstream.scss",
            "$b: old value;\n\n@function get-b() {@return $b}\n",
        )
        .mock_file(
            "variable_assignment/top_level/_midstream.scss",
            "@forward \"upstream\" as d-*;\n",
        )
        .mock_file(
            "variable_assignment/top_level/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file(
            "variable_use/_midstream.scss",
            "@forward \"upstream\" as d-*;\n",
        )
        .mock_file("variable_use/_upstream.scss", "$c: e;\n")
}

#[test]
fn different_separator() {
    let runner = runner().with_cwd("different_separator");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$d-c}\n"),
        "a {\
         \n  b: e;\
         \n}\n"
    );
}
#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {b: midstream.d-c()}\n"),
        "a {\
         \n  b: e;\
         \n}\n"
    );
}
#[test]
fn hide() {
    let runner = runner().with_cwd("hide");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \n@include midstream.b-a;\n"),
        "c {\
         \n  d: e;\
         \n}\n"
    );
}
#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \n@include midstream.b-a;\n"),
        "c {\
         \n  d: e;\
         \n}\n"
    );
}
mod no_conflict {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("no_conflict")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.c-d()}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {@include midstream.b-c}\n"),
            "a {\
         \n  d: e;\
         \n}\n"
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c-d}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
mod show {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("show")
    }

    #[test]
    fn different_separator() {
        let runner = runner().with_cwd("different_separator");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \n@include midstream.b-a;\n"),
            "c {\
         \n  d: e;\
         \n}\n"
        );
    }
    #[test]
    fn same_separator() {
        let runner = runner().with_cwd("same_separator");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \n@include midstream.b-a;\n"),
            "c {\
         \n  d: e;\
         \n}\n"
        );
    }
}
mod variable_assignment {
    #[allow(unused)]
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
             \n  midstream.$d-b: new value;\n\
             \n  c: midstream.d-get-b();\
             \n}\n"
        ),
        "a {\
         \n  c: new value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn top_level() {
        let runner = runner().with_cwd("top_level");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nmidstream.$d-a: new value;\n\
             \nb {c: midstream.d-get-a()};\n"),
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
             \na {b: midstream.$d-c}\n"),
        "a {\
         \n  b: e;\
         \n}\n"
    );
}
