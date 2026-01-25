//! Tests auto-converted from "sass-spec/spec/css/plain/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("function")
        .mock_file(
            "lowercase/parameter/plain.css",
            "@function --a(--b <color>) {result: c}\n",
        )
        .mock_file(
            "lowercase/result/characters/plain.css",
            "@function --a() {\n  result: {}#&%^*;\n}\n",
        )
        .mock_file(
            "lowercase/result/sass_script/plain.css",
            "@function --a() {\n  result: $b;\n}\n",
        )
        .mock_file(
            "lowercase/returns/plain.css",
            "@function --a() returns <ident> {result: b}\n",
        )
        .mock_file(
            "result/uppercase/characters/plain.css",
            "@function --a() {\n  RESULT: {}#&%^*;\n}\n",
        )
        .mock_file(
            "result/uppercase/sass_script/plain.css",
            "@function --a() {\n  RESULT: $b;\n}\n",
        )
        .mock_file(
            "uppercase/result/characters/plain.css",
            "@FUNCTION --a() {\n  result: {}#&%^*;\n}\n",
        )
        .mock_file(
            "uppercase/result/sass_script/plain.css",
            "@FUNCTION --a() {\n  result: $b;\n}\n",
        )
}

mod lowercase {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("lowercase")
    }

    #[test]
    fn parameter() {
        let runner = runner().with_cwd("parameter");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "@function --a(--b <color>) {\
         \n  result: c;\
         \n}\n"
        );
    }
    mod result {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("result")
        }

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            let runner = runner().with_cwd("characters");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@function --a() {\
         \n  result: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass_script() {
            let runner = runner().with_cwd("sass_script");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@function --a() {\
         \n  result: $b;\
         \n}\n"
            );
        }
    }
    #[test]
    fn returns() {
        let runner = runner().with_cwd("returns");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "@function --a() returns <ident> {\
         \n  result: b;\
         \n}\n"
        );
    }
}
mod result {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("result")
    }

    mod uppercase {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("uppercase")
        }

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            let runner = runner().with_cwd("characters");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@function --a() {\
         \n  RESULT: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass_script() {
            let runner = runner().with_cwd("sass_script");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@function --a() {\
         \n  RESULT: $b;\
         \n}\n"
            );
        }
    }
}
mod uppercase {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("uppercase")
    }

    mod result {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("result")
        }

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            let runner = runner().with_cwd("characters");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@FUNCTION --a() {\
         \n  result: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass_script() {
            let runner = runner().with_cwd("sass_script");
            assert_eq!(
                runner.ok("@use \"plain\";\n"),
                "@FUNCTION --a() {\
         \n  result: $b;\
         \n}\n"
            );
        }
    }
}
