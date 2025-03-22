//! Tests auto-converted from "sass-spec/spec/directives/use/with/core_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("core_module")
        .mock_file(
            "indirect/forward/_other.scss",
            "@forward \"sass:color\";\n\n$c: d !default;\n",
        )
        .mock_file(
            "indirect/use/_other.scss",
            "@use \"sass:color\";\n\n$c: d !default;\n",
        )
}

mod indirect {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("indirect")
    }

    #[test]
    fn forward() {
        let runner = runner().with_cwd("forward");
        assert_eq!(
            runner.ok("// Regression test for sass/dart-sass#838.\
             \n@use \"other\" with ($c: e);\n\
             \na {b: other.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        let runner = runner().with_cwd("use");
        assert_eq!(
            runner.ok("// Regression test for sass/dart-sass#838.\
             \n@use \"other\" with ($c: e);\n\
             \na {b: other.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
