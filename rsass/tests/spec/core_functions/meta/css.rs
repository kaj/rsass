//! Tests auto-converted from "sass-spec/spec/core_functions/meta/css.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("css")
        .mock_file("direct/get_module/_other.scss", "a {b: c}\n")
        .mock_file("direct/load/_other.scss", "a {b: c}\n")
        .mock_file("indirect/get_module/_midstream.scss", "@use \"sass:meta\";\n@use \"upstream\";\n$module: meta.get-module(\"upstream\");\n")
        .mock_file("indirect/get_module/_upstream.scss", "a {b: c}\n")
        .mock_file("indirect/load/_midstream.scss", "@use \"sass:meta\";\n$module: meta.load(\"upstream\");\n")
        .mock_file("indirect/load/_upstream.scss", "a {b: c}\n")
        .mock_file("named/_other.scss", "a {b: c}\n")
        .mock_file("nested/_other.scss", "b {c: d}\n")
        .mock_file("string/_other.scss", "a {b: c}\n")
}

mod direct {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("direct")
    }

    #[test]
    #[ignore] // unexepected error
    fn get_module() {
        let runner = runner().with_cwd("get_module");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.css(meta.get-module(\"other\"));\n"),
            "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn load() {
        let runner = runner().with_cwd("load");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.css(meta.load(\"other\"));\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    mod argument {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("argument")
        }

        #[test]
        #[ignore] // wrong error
        fn too_few() {
            let runner = runner().with_cwd("too_few");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@include meta.css();\n"
                ),
                "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | @include meta.css();\
         \n  | ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin css($module) {\
         \n  |        ============ declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn too_many() {
            let runner = runner().with_cwd("too_many");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@include meta.css(meta.get-module(\"meta\"), a);\n"
                ),
                "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | @include meta.css(meta.get-module(\"meta\"), a);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin css($module) {\
         \n  |        ============ declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn test_type() {
            let runner = runner().with_cwd("type");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@include meta.css(1);\n"
                ),
                "Error: $module: 1 is neither a string nor a module reference.\
         \n  ,\
         \n2 | @include meta.css(1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
}
mod indirect {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("indirect")
    }

    #[test]
    #[ignore] // unexepected error
    fn get_module() {
        let runner = runner().with_cwd("get_module");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\";\
             \n@include meta.css(midstream.$module);\n"),
            "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn load() {
        let runner = runner().with_cwd("load");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\";\
             \n@include meta.css(midstream.$module);\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@include meta.css($module: meta.load(\"other\"));\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \na {@include meta.css(meta.load(\"other\"))}\n"),
        "a b {\
         \n  c: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn string() {
    let runner = runner().with_cwd("string");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.css(\"other\");\n"),
        "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
    );
}
