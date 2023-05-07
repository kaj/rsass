//! Tests auto-converted from "sass-spec/spec/css/plain/media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("media")
        .mock_file(
            "logic/and/mixed_case/plain.css",
            "@media (a) AnD (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and/multiple/plain.css",
            "@media (a) and (b) and (c) and (d) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and/no_whitespace_before/plain.css",
            "@media (a)and (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and/one/plain.css",
            "@media (a) and (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and_not/after_type_and_modifier/plain.css",
            "@media only a and not (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and_not/lowercase/plain.css",
            "@media a and not (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and_not/mixed_case/plain.css",
            "@media a AnD nOt (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/not/lowercase/plain.css",
            "@media not (a) {x {y: z}}\n",
        )
        .mock_file(
            "logic/not/mixed_case/plain.css",
            "@media NoT (a) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or/mixed_case/plain.css",
            "@media (a) oR (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or/multiple/plain.css",
            "@media (a) or (b) or (c) or (d) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or/no_whitespace_before/plain.css",
            "@media (a)or (b) {x {y: z}}\n",
        )
        .mock_file("logic/or/one/plain.css", "@media (a) or (b) {x {y: z}}\n")
}

mod logic {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("logic")
    }

    mod and {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("and")
        }

        #[test]
        fn mixed_case() {
            let runner = runner().with_cwd("mixed_case");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn multiple() {
            let runner = runner().with_cwd("multiple");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) and (b) and (c) and (d) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn no_whitespace_before() {
            let runner = runner().with_cwd("no_whitespace_before");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn one() {
            let runner = runner().with_cwd("one");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod and_not {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("and_not")
        }

        #[test]
        fn after_type_and_modifier() {
            let runner = runner().with_cwd("after_type_and_modifier");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media only a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn lowercase() {
            let runner = runner().with_cwd("lowercase");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn mixed_case() {
            let runner = runner().with_cwd("mixed_case");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod not {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("not")
        }

        #[test]
        fn lowercase() {
            let runner = runner().with_cwd("lowercase");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn mixed_case() {
            let runner = runner().with_cwd("mixed_case");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod or {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("or")
        }

        #[test]
        fn mixed_case() {
            let runner = runner().with_cwd("mixed_case");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn multiple() {
            let runner = runner().with_cwd("multiple");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) or (b) or (c) or (d) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn no_whitespace_before() {
            let runner = runner().with_cwd("no_whitespace_before");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn one() {
            let runner = runner().with_cwd("one");
            assert_eq!(
                runner.ok("@import \'plain\';\n"),
                "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
}
