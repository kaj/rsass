//! Tests auto-converted from "sass-spec/spec/directives/at_root.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("at_root")
        .mock_file(
            "load_css/other.scss",
            "@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file(
            "nested_import/with_builtin_use/other.scss",
            "@use \"sass:math\";\n\n@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file(
            "nested_import/with_no_use/other.scss",
            "@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file(
            "nested_import/with_user_use/other.scss",
            "@use \"used\";\n\n@at-root {\n  b {\n    c: d;\n  }\n}\n",
        )
        .mock_file("nested_import/with_user_use/used.scss", "// nothing\n")
}

mod comment {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("comment")
    }

    mod after_colon {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("after_colon")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root (without: /**/ media) {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root (without: //\
             \n  media) {}\n"),
                ""
            );
        }
    }
    mod after_open_paren {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("after_open_paren")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root (/**/ without: media) {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root (//\
             \n  without: media) {}\n"),
                ""
            );
        }
    }
    mod after_query {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("after_query")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root (without: media) /**/ {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root (without: media) //\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_close_paren {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("before_close_paren")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root (without: media /**/) {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root (without: media //\
             \n  ) {}\n"),
                ""
            );
        }
    }
    mod before_colon {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("before_colon")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root (without /**/ : media) {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root (without //\
             \n  : media) {}\n"),
                ""
            );
        }
    }
    mod before_query {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("before_query")
        }

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root /**/ (without: media) {}\n"), "");
        }
        #[test]
        #[ignore] // unexepected error
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root //\
             \n  (without: media) {}\n"),
                ""
            );
        }
    }
    mod no_query {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("no_query")
        }

        #[test]
        fn loud() {
            let runner = runner().with_cwd("loud");
            assert_eq!(runner.ok("@at-root /**/ {}\n"), "");
        }
        #[test]
        fn silent() {
            let runner = runner().with_cwd("silent");
            assert_eq!(
                runner.ok("@at-root //\
             \n  {}\n"),
                ""
            );
        }
    }
}
mod keyframes {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("keyframes")
    }

    #[test]
    #[ignore] // unexepected error
    fn all() {
        let runner = runner().with_cwd("all");
        assert_eq!(
            runner.ok("@keyframes a {\
             \n  @at-root (without: all) {\
             \n    b {c: d}\
             \n  }\
             \n}\n"),
            "@keyframes a {}\
         \nb {\
         \n  c: d;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn load_css() {
    let runner = runner().with_cwd("load_css");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\n\
             \na {\
             \n  @include meta.load-css(\"other\");\
             \n}\n"),
        "a b {\
         \n  c: d;\
         \n}\n"
    );
}
mod nested_import {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nested_import")
    }

    #[test]
    fn with_builtin_use() {
        let runner = runner().with_cwd("with_builtin_use");
        assert_eq!(
            runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
            "b {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn with_no_use() {
        let runner = runner().with_cwd("with_no_use");
        assert_eq!(
            runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
            "b {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    fn with_user_use() {
        let runner = runner().with_cwd("with_user_use");
        assert_eq!(
            runner.ok("a {\
             \n  @import \"other\";\
             \n}\n"),
            "b {\
         \n  c: d;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn property_only() {
    let runner = runner().with_cwd("property_only");
    assert_eq!(
        runner.ok("@media print {\
             \n  a {\
             \n    @at-root (without: media) {\
             \n      b: c;\
             \n    }\
             \n  }\
             \n}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod sass {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("sass")
    }

    mod empty {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("empty")
        }
    }
}
