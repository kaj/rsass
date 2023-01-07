//! Tests auto-converted from "sass-spec/spec/directives/import/implicit_dependencies.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("implicit_dependencies")
        .mock_file(
            "forwarded_first/no_use/first.import.scss",
            "@forward \"first\";\n",
        )
        .mock_file("forwarded_first/no_use/first.scss", "$variable: value;\n")
        .mock_file(
            "forwarded_first/no_use/second.scss",
            "a {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "forwarded_first/use_in_both/first.import.scss",
            "@forward \"first\";\n",
        )
        .mock_file(
            "forwarded_first/use_in_both/first.scss",
            "@use \"sass:math\";\n\n$variable: value;\n",
        )
        .mock_file(
            "forwarded_first/use_in_both/second.scss",
            "@use \"sass:math\";\n\na {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "forwarded_first/use_in_first/first.import.scss",
            "@forward \"first\";\n",
        )
        .mock_file(
            "forwarded_first/use_in_first/first.scss",
            "$variable: value;\n",
        )
        .mock_file(
            "forwarded_first/use_in_first/second.scss",
            "@use \"sass:math\";\n\na {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "forwarded_first/use_in_second/first.import.scss",
            "@forward \"first\";\n",
        )
        .mock_file(
            "forwarded_first/use_in_second/first.scss",
            "@use \"sass:math\";\n\n$variable: value;\n",
        )
        .mock_file(
            "forwarded_first/use_in_second/second.scss",
            "a {\n  b: $variable;\n}\n",
        )
        .mock_file("no_forward/no_use/first.scss", "$variable: value;\n")
        .mock_file(
            "no_forward/no_use/second.scss",
            "a {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "no_forward/use_in_both/first.scss",
            "@use \"sass:math\";\n\n$variable: value;\n",
        )
        .mock_file(
            "no_forward/use_in_both/second.scss",
            "@use \"sass:math\";\n\na {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "no_forward/use_in_first/first.scss",
            "@use \"sass:math\";\n\n$variable: value;\n",
        )
        .mock_file(
            "no_forward/use_in_first/second.scss",
            "a {\n  b: $variable;\n}\n",
        )
        .mock_file(
            "no_forward/use_in_second/first.scss",
            "$variable: value;\n",
        )
        .mock_file(
            "no_forward/use_in_second/second.scss",
            "@use \"sass:math\";\n\na {\n  b: $variable;\n}\n",
        )
}

mod forwarded_first {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("forwarded_first")
    }

    #[test]
    fn no_use() {
        let runner = runner().with_cwd("no_use");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_both() {
        let runner = runner().with_cwd("use_in_both");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_first() {
        let runner = runner().with_cwd("use_in_first");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_second() {
        let runner = runner().with_cwd("use_in_second");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
}
mod no_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("no_forward")
    }

    #[test]
    fn no_use() {
        let runner = runner().with_cwd("no_use");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_both() {
        let runner = runner().with_cwd("use_in_both");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_first() {
        let runner = runner().with_cwd("use_in_first");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
    #[test]
    fn use_in_second() {
        let runner = runner().with_cwd("use_in_second");
        assert_eq!(
            runner.ok("@import \"first\";\
             \n@import \"second\";\n"),
            "a {\
         \n  b: value;\
         \n}\n"
        );
    }
}
