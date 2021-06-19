//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/override.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "override/function/_midstream1.scss",
            "@forward \"upstream1\";\n",
        )
        .mock_file(
            "override/function/_midstream2.scss",
            "@forward \"upstream2\";\n",
        )
        .mock_file(
            "override/function/_upstream1.scss",
            "@function b() {@return 1};\n",
        )
        .mock_file(
            "override/function/_upstream2.scss",
            "@function b() {@return 2};\n",
        )
        .mock_file(
            "override/mixin/_midstream1.scss",
            "@forward \"upstream1\";\n",
        )
        .mock_file(
            "override/mixin/_midstream2.scss",
            "@forward \"upstream2\";\n",
        )
        .mock_file("override/mixin/_upstream1.scss", "@mixin a {b: 1};\n")
        .mock_file("override/mixin/_upstream2.scss", "@mixin a {b: 2};\n")
        .mock_file(
            "override/variable/_midstream1.scss",
            "@forward \"upstream1\";\n",
        )
        .mock_file(
            "override/variable/_midstream2.scss",
            "@forward \"upstream2\";\n",
        )
        .mock_file("override/variable/_upstream1.scss", "$b: 1;\n")
        .mock_file("override/variable/_upstream2.scss", "$b: 2;\n")
}

mod test_override {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("override")
    }

    #[test]
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@import \"midstream1\";\
             \nafter-first {a: b()}\n\
             \n@import \"midstream2\";\
             \nafter-second {a: b()}\n"),
            "after-first {\
         \n  a: 1;\
         \n}\
         \nafter-second {\
         \n  a: 2;\
         \n}\n"
        );
    }
    #[test]
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@import \"midstream1\";\
             \nafter-first {@include a}\n\
             \n@import \"midstream2\";\
             \nafter-second {@include a}\n"),
            "after-first {\
         \n  b: 1;\
         \n}\
         \nafter-second {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.ok("@import \"midstream1\";\
             \nafter-first {a: $b}\n\
             \n@import \"midstream2\";\
             \nafter-second {a: $b}\n"),
            "after-first {\
         \n  a: 1;\
         \n}\
         \nafter-second {\
         \n  a: 2;\
         \n}\n"
        );
    }
}
