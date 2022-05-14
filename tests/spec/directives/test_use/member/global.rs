//! Tests auto-converted from "sass-spec/spec/directives/use/member/global.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("global")
        .mock_file(
            "function/other.scss",
            "@function member() {@return value}\n",
        )
        .mock_file("mixin/other.scss", "@mixin member() {a {b: c}}\n")
        .mock_file("multiple/left.scss", "$left: left;\n")
        .mock_file("multiple/right.scss", "$right: right;\n")
        .mock_file(
            "no_conflict/function/_other.scss",
            "@function c() {@return d}\n",
        )
        .mock_file("no_conflict/mixin/_other.scss", "@mixin b {c: d}\n")
        .mock_file("no_conflict/variable/_other.scss", "$c: d;\n")
        .mock_file(
            "variable_assignment/nested/global/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file(
            "variable_assignment/nested/local/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file(
            "variable_assignment/top_level/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file("variable_use/other.scss", "$member: value;\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("@use \"other\" as *;\n\
             \na {b: member()}\n"),
        "a {\
         \n  b: value;\
         \n}\n"
    );
}
#[test]
fn mixin() {
    let runner = runner().with_cwd("mixin");
    assert_eq!(
        runner.ok("@use \"other\" as *;\n\
             \n@include member;\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn multiple() {
    let runner = runner().with_cwd("multiple");
    assert_eq!(
        runner.ok("@use \"left\" as *;\
             \n@use \"right\" as *;\n\
             \na {\
             \n  left: $left;\
             \n  right: $right;\
             \n}\n"),
        "a {\
         \n  left: left;\
         \n  right: right;\
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
            runner.ok("@use \"other\" as *;\
             \n@use \"other\" as *;\n\
             \na {b: c()}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"other\" as *;\
             \n@use \"other\" as *;\n\
             \na {@include b}\n"),
            "a {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.ok("@use \"other\" as *;\
             \n@use \"other\" as *;\n\
             \na {b: $c}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
}
mod variable_assignment {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("variable_assignment")
    }

    mod nested {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("nested")
        }

        #[test]
        #[ignore] // wrong result
        fn global() {
            let runner = runner().with_cwd("global");
            assert_eq!(
        runner.ok(
            "@use \"other\" as *;\n\
             \na {\
             \n  // A nested variable assignment that doesn\'t have a namespace but is !global\
             \n  // assigns to a global module\'s variable if one exists.\
             \n  $member: new value !global;\n\
             \n  b: get-member();\
             \n}\n"
        ),
        "a {\
         \n  b: new value;\
         \n}\n"
    );
        }
        #[test]
        fn local() {
            let runner = runner().with_cwd("local");
            assert_eq!(
        runner.ok(
            "@use \"other\" as *;\n\
             \na {\
             \n  // A nested variable assignment that doesn\'t have a namespace and isn\'t\
             \n  // !global creates a new local variable rather than assigning to a variable\
             \n  // imported from a module.\
             \n  $member: new value;\n\
             \n  b: get-member();\
             \n}\n"
        ),
        "a {\
         \n  b: value;\
         \n}\n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn top_level() {
        let runner = runner().with_cwd("top_level");
        assert_eq!(
            runner.ok("@use \"other\" as *;\n\
             \n$member: new value;\n\
             \na {b: get-member()}\n"),
            "a {\
         \n  b: new value;\
         \n}\n"
        );
    }
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.ok("@use \"other\" as *;\n\
             \na {b: $member}\n"),
        "a {\
         \n  b: value;\
         \n}\n"
    );
}
