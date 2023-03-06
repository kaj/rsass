//! Tests auto-converted from "sass-spec/spec/css/font-face.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("font-face")
        .mock_file(
            "bubble/loaded/import/upstream.scss",
            "@font-face { a: b }\n",
        )
        .mock_file(
            "bubble/loaded/meta-load-css/upstream.scss",
            "@font-face { a: b }\n",
        )
}

mod bubble {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("bubble")
    }

    #[test]
    #[ignore] // wrong result
    fn deeply_nested() {
        let runner = runner().with_cwd("deeply-nested");
        assert_eq!(
            runner.ok("a { b { c { @font-face { e: f } g: h; } } }\n"),
            "a b c {\
         \n  g: h;\
         \n}\
         \n@font-face {\
         \n  e: f;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn empty() {
        let runner = runner().with_cwd("empty");
        assert_eq!(
            runner.ok("a {\
             \n  @font-face {/**/}\
             \n}\n"),
            "@font-face { /**/ }\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn in_mixin() {
        let runner = runner().with_cwd("in-mixin");
        assert_eq!(
            runner.ok("@mixin a {\
             \n  @font-face { b: c }\
             \n}\
             \nd {\
             \n  e: f;\
             \n  @include a;\
             \n}\n"),
            "d {\
         \n  e: f;\
         \n}\
         \n@font-face {\
         \n  b: c;\
         \n}\n"
        );
    }
    mod loaded {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("loaded")
        }

        #[test]
        #[ignore] // wrong result
        fn import() {
            let runner = runner().with_cwd("import");
            assert_eq!(
                runner.ok("c {\
             \n  @import \'upstream\';\
             \n  d: e;\
             \n}\n"),
                "c {\
         \n  d: e;\
         \n}\
         \n@font-face {\
         \n  a: b;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn meta_load_css() {
            let runner = runner().with_cwd("meta-load-css");
            assert_eq!(
                runner.ok("@use \'sass:meta\';\n\
             \nc {\
             \n  @include meta.load-css(\'upstream\');\
             \n  d: e;\
             \n}\n"),
                "c {\
         \n  d: e;\
         \n}\
         \n@font-face {\
         \n  a: b;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn rules() {
        let runner = runner().with_cwd("rules");
        assert_eq!(
            runner.ok("a {\
             \n  b: c;\
             \n  @font-face { d: e }\
             \n}\n"),
            "a {\
         \n  b: c;\
         \n}\
         \n@font-face {\
         \n  d: e;\
         \n}\n"
        );
    }
}
