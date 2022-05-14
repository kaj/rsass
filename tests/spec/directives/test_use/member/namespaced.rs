//! Tests auto-converted from "sass-spec/spec/directives/use/member/namespaced.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("namespaced")
        .mock_file(
            "default/basename/foo/baz/qux/other.scss",
            "$variable: value;\n",
        )
        .mock_file(
            "default/function/other.scss",
            "@function member() {@return value}\n",
        )
        .mock_file("default/mixin/other.scss", "@mixin member() {a {b: c}}\n")
        .mock_file(
            "default/variable_assignment/in_declaration/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file(
            "default/variable_assignment/in_function/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file(
            "default/variable_assignment/nested/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file(
            "default/variable_assignment/top_level/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file("default/variable_use/other.scss", "$member: value;\n")
        .mock_file(
            "default/without_extensions/other.foo.bar.baz.scss",
            "$variable: value;\n",
        )
        .mock_file(
            "default/without_underscore/_other.scss",
            "$variable: value;\n",
        )
        .mock_file(
            "explicit/function/other.scss",
            "@function member() {@return value}\n",
        )
        .mock_file(
            "explicit/mixin/other.scss",
            "@mixin member() {a {b: c}}\n",
        )
        .mock_file(
            "explicit/variable_assignment/other.scss",
            "$member: value;\n\n@function get-member() {@return $member}\n",
        )
        .mock_file("explicit/variable_use/other.scss", "$member: value;\n")
}

mod default {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("default")
    }

    #[test]
    #[ignore] // unexepected error
    fn basename() {
        let runner = runner().with_cwd("basename");
        assert_eq!(
        runner.ok(
            "// Only the basename of the URL is used for the namespace. Previous components\
             \n// are discarded.\
             \n@use \"foo/bar/../baz/qux/other\";\n\
             \na {b: other.$variable}\n"
        ),
        "a {\
         \n  b: value;\
         \n}\n"
    );
    }
    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@use \"other\";\n\
             \na {b: other.member()}\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"other\";\n\
             \n@include other.member;\n"),
            "a {\
         \n  b: c;\
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
        fn in_declaration() {
            let runner = runner().with_cwd("in_declaration");
            assert_eq!(
        runner.ok(
            "@use \"other\";\n\
             \na {\
             \n  b: {\
             \n    // Test assignments within a declaration specially, because declarations\
             \n    // disallow style rules and variable assignments need to be disambiguated\
             \n    // with those.\
             \n    other.$member: new value;\n\
             \n    c: other.get-member();\
             \n  }\
             \n}\n"
        ),
        "a {\
         \n  b-c: new value;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_function() {
            let runner = runner().with_cwd("in_function");
            assert_eq!(
        runner.ok(
            "@use \"other\";\n\
             \n@function a() {\
             \n  // Test assignments within a function specially, because functions disallow\
             \n  // property declarations and variable assignments need to be disambiguated\
             \n  // with those.\
             \n  other.$member: new value;\n\
             \n  @return other.get-member();\
             \n}\n\
             \nb {c: a()}\n"
        ),
        "b {\
         \n  c: new value;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn nested() {
            let runner = runner().with_cwd("nested");
            assert_eq!(
        runner.ok(
            "@use \"other\";\n\
             \na {\
             \n  // Namespaced assignments always assign to the other module\'s variable, even\
             \n  // if they\'re nested in a block scope.\
             \n  other.$member: new value;\n\
             \n  b: other.get-member();\
             \n}\n"
        ),
        "a {\
         \n  b: new value;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn top_level() {
            let runner = runner().with_cwd("top_level");
            assert_eq!(
                runner.ok("@use \"other\";\n\
             \nother.$member: new value;\n\
             \na {b: other.get-member()};\n"),
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
            runner.ok("@use \"other\";\n\
             \na {b: other.$member}\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn without_extensions() {
        let runner = runner().with_cwd("without_extensions");
        assert_eq!(
        runner.ok(
            "// All extensions on the URL are discarded before determining the namespace.\
             \n@use \"other.foo.bar.baz.scss\";\n\
             \na {b: other.$variable}\n"
        ),
        "a {\
         \n  b: value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn without_underscore() {
        let runner = runner().with_cwd("without_underscore");
        assert_eq!(
        runner.ok(
            "// A single leading underscore is removed before determining the namespace.\
             \n@use \"_other\";\n\
             \na {b: other.$variable}\n"
        ),
        "a {\
         \n  b: value;\
         \n}\n"
    );
    }
}
mod explicit {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("explicit")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@use \"other\" as o;\n\
             \na {b: o.member()}\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"other\" as o;\n\
             \n@include o.member;\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn variable_assignment() {
        let runner = runner().with_cwd("variable_assignment");
        assert_eq!(
            runner.ok("@use \"other\" as o;\n\
             \no.$member: new value;\n\
             \na {b: o.get-member()}\n"),
            "a {\
         \n  b: new value;\
         \n}\n"
        );
    }
    #[test]
    fn variable_use() {
        let runner = runner().with_cwd("variable_use");
        assert_eq!(
            runner.ok("@use \"other\" as o;\n\
             \na {b: o.$member}\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
}
