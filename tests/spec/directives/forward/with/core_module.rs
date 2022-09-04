//! Tests auto-converted from "sass-spec/spec/directives/forward/with/core_module.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("core_module")
        .mock_file("indirect/forward/_midstream.scss", "// Regression test for sass/dart-sass#838.\n@forward \"upstream\" with ($c: e);\n")
        .mock_file("indirect/forward/_upstream.scss", "@forward \"sass:color\";\n\n$c: d !default;\n")
        .mock_file("indirect/use/_midstream.scss", "// Regression test for sass/dart-sass#838.\n@forward \"upstream\" with ($c: e);\n")
        .mock_file("indirect/use/_upstream.scss", "@use \"sass:color\";\n\n$c: d !default;\n")
}

mod indirect {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("indirect")
    }

    #[test]
    fn forward() {
        let runner = runner().with_cwd("forward");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        let runner = runner().with_cwd("use");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \na {b: midstream.$c}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
