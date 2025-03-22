//! Tests auto-converted from "sass-spec/spec/directives/forward/member/visibility.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("visibility")
        .mock_file(
            "hide/mixin/_midstream.scss",
            "@forward \"upstream\" hide b;\n",
        )
        .mock_file("hide/mixin/_upstream.scss", "@mixin a() {c {d: e}}\n")
        .mock_file(
            "hide/variable_assignment/_midstream.scss",
            "@forward \"upstream\" hide d;\n",
        )
        .mock_file(
            "hide/variable_assignment/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file(
            "hide/variable_use/_midstream.scss",
            "@forward \"upstream\" hide d;\n",
        )
        .mock_file("hide/variable_use/_upstream.scss", "$c: e;\n")
        .mock_file(
            "hide/wrong_type/mixin/_midstream.scss",
            "@forward \"upstream\" hide $a;\n",
        )
        .mock_file(
            "hide/wrong_type/mixin/_upstream.scss",
            "@mixin a() {c {d: e}}\n",
        )
        .mock_file(
            "hide/wrong_type/variable_use/_midstream.scss",
            "@forward \"upstream\" hide c;\n",
        )
        .mock_file("hide/wrong_type/variable_use/_upstream.scss", "$c: e;\n")
        .mock_file(
            "show/mixin/_midstream.scss",
            "@forward \"upstream\" show a;\n",
        )
        .mock_file("show/mixin/_upstream.scss", "@mixin a() {b {c: d}}\n")
        .mock_file(
            "show/variable_assignment/_midstream.scss",
            "@forward \"upstream\" show $a, get-a;\n",
        )
        .mock_file(
            "show/variable_assignment/_upstream.scss",
            "$a: old value;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file(
            "show/variable_use/_midstream.scss",
            "@forward \"upstream\" show $c;\n",
        )
        .mock_file("show/variable_use/_upstream.scss", "$c: d;\n")
}

mod hide {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("hide")
    }

    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \n@include midstream.a;\n"),
            "c {\
         \n  d: e;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn variable_assignment() {
        let runner = runner().with_cwd("variable_assignment");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nmidstream.$a: new value;\n\
             \nb {c: midstream.get-a()};\n"),
            "b {\
         \n  c: new value;\
         \n}\n"
        );
    }
    #[test]
    fn variable_use() {
        let runner = runner().with_cwd("variable_use");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    mod wrong_type {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("wrong_type")
        }

        #[test]
        fn mixin() {
            let runner = runner().with_cwd("mixin");
            assert_eq!(
                runner.ok("@use \"midstream\";\n\
             \n@include midstream.a;\n"),
                "c {\
         \n  d: e;\
         \n}\n"
            );
        }
        #[test]
        fn variable_use() {
            let runner = runner().with_cwd("variable_use");
            assert_eq!(
                runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
    }
}
mod show {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("show")
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
    #[test]
    #[ignore] // wrong result
    fn variable_assignment() {
        let runner = runner().with_cwd("variable_assignment");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nmidstream.$a: new value;\n\
             \nb {c: midstream.get-a()};\n"),
            "b {\
         \n  c: new value;\
         \n}\n"
        );
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
}
