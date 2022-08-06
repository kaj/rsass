//! Tests auto-converted from "sass-spec/spec/css/plain/error/media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("media")
        .mock_file(
            "logic/and_after/or/plain.css",
            "@media (a) or (b) and (c) {x {y: z}}\n",
        )
        .mock_file(
            "logic/and_after/type_and_not/plain.css",
            "@media a and not (b) and (c) {x {y: z}}\n",
        )
        .mock_file(
            "logic/nothing_after/and/after_paren/plain.css",
            "@media (a) and {x {y: z}}\n",
        )
        .mock_file(
            "logic/nothing_after/and/after_type/plain.css",
            "@media a and {x {y: z}}\n",
        )
        .mock_file(
            "logic/nothing_after/and_not/plain.css",
            "@media a and not {x {y: z}}\n",
        )
        .mock_file(
            "logic/nothing_after/not/plain.css",
            "@media not {x {y: z}}\n",
        )
        .mock_file(
            "logic/nothing_after/or/plain.css",
            "@media (a) or {x {y: z}}\n",
        )
        .mock_file(
            "logic/or_after/and/plain.css",
            "@media (a) and (b) or (c) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or_after/type/plain.css",
            "@media a or (b) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or_after/type_and_not/plain.css",
            "@media a and not (b) or (c) {x {y: z}}\n",
        )
        .mock_file(
            "logic/or_after/type_then_and/plain.css",
            "@media a and (b) or (c) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/and/after_type/plain.css",
            "@media a and(b) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/and/first/plain.css",
            "@media (a) and(b) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/and/later/plain.css",
            "@media (a) and (b) and(c) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/and_not/type/plain.css",
            "@media a and not(b) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/and_not/type_and_modifier/plain.css",
            "@media only a and not(b) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/not/plain.css",
            "@media not(a) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/or/first/plain.css",
            "@media (a) or(b) {x {y: z}}\n",
        )
        .mock_file(
            "missing_whitespace/or/later/plain.css",
            "@media (a) or (b) or(c) {x {y: z}}\n",
        )
}

mod logic {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("logic")
    }

    mod and_after {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("and_after")
        }

        #[test]
        #[ignore] // missing error
        fn or() {
            let runner = runner().with_cwd("or");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media (a) or (b) and (c) {x {y: z}}\
         \n  |                   ^\
         \n  \'\
         \n  plain.css 1:19  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn type_and_not() {
            let runner = runner().with_cwd("type_and_not");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and not (b) and (c) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 1:22  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    mod nothing_after {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("nothing_after")
        }

        mod and {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("and")
            }

            #[test]
            #[ignore] // missing error
            fn after_paren() {
                let runner = runner().with_cwd("after_paren");
                assert_eq!(
                    runner.err("@import \'plain\';\n"),
                    "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media (a) and {x {y: z}}\
         \n  |                ^\
         \n  \'\
         \n  plain.css 1:16  @import\
         \n  input.scss 1:9  root stylesheet",
                );
            }
            #[test]
            #[ignore] // missing error
            fn after_type() {
                let runner = runner().with_cwd("after_type");
                assert_eq!(
                    runner.err("@import \'plain\';\n"),
                    "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media a and {x {y: z}}\
         \n  |              ^\
         \n  \'\
         \n  plain.css 1:14  @import\
         \n  input.scss 1:9  root stylesheet",
                );
            }
        }
        #[test]
        #[ignore] // missing error
        fn and_not() {
            let runner = runner().with_cwd("and_not");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media a and not {x {y: z}}\
         \n  |                  ^\
         \n  \'\
         \n  plain.css 1:18  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn not() {
            let runner = runner().with_cwd("not");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media not {x {y: z}}\
         \n  |            ^\
         \n  \'\
         \n  plain.css 1:12  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn or() {
            let runner = runner().with_cwd("or");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected media condition in parentheses.\
         \n  ,\
         \n1 | @media (a) or {x {y: z}}\
         \n  |               ^\
         \n  \'\
         \n  plain.css 1:15  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    mod or_after {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("or_after")
        }

        #[test]
        #[ignore] // missing error
        fn and() {
            let runner = runner().with_cwd("and");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media (a) and (b) or (c) {x {y: z}}\
         \n  |                    ^\
         \n  \'\
         \n  plain.css 1:20  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn test_type() {
            let runner = runner().with_cwd("type");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a or (b) {x {y: z}}\
         \n  |             ^\
         \n  \'\
         \n  plain.css 1:13  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn type_and_not() {
            let runner = runner().with_cwd("type_and_not");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and not (b) or (c) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 1:22  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn type_then_and() {
            let runner = runner().with_cwd("type_then_and");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: expected \"{\".\
         \n  ,\
         \n1 | @media a and (b) or (c) {x {y: z}}\
         \n  |                  ^\
         \n  \'\
         \n  plain.css 1:18  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
}
mod missing_whitespace {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("missing_whitespace")
    }

    mod and {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("and")
        }

        #[test]
        #[ignore] // missing error
        fn after_type() {
            let runner = runner().with_cwd("after_type");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media a and(b) {x {y: z}}\
         \n  |             ^\
         \n  \'\
         \n  plain.css 1:13  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn first() {
            let runner = runner().with_cwd("first");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) and(b) {x {y: z}}\
         \n  |               ^\
         \n  \'\
         \n  plain.css 1:15  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn later() {
            let runner = runner().with_cwd("later");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) and (b) and(c) {x {y: z}}\
         \n  |                       ^\
         \n  \'\
         \n  plain.css 1:23  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    mod and_not {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("and_not")
        }

        #[test]
        #[ignore] // missing error
        fn test_type() {
            let runner = runner().with_cwd("type");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media a and not(b) {x {y: z}}\
         \n  |                 ^\
         \n  \'\
         \n  plain.css 1:17  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn type_and_modifier() {
            let runner = runner().with_cwd("type_and_modifier");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media only a and not(b) {x {y: z}}\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 1:22  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        let runner = runner().with_cwd("not");
        assert_eq!(
            runner.err("@import \'plain\';\n"),
            "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media not(a) {x {y: z}}\
         \n  |           ^\
         \n  \'\
         \n  plain.css 1:11  @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    mod or {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("or")
        }

        #[test]
        #[ignore] // missing error
        fn first() {
            let runner = runner().with_cwd("first");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) or(b) {x {y: z}}\
         \n  |              ^\
         \n  \'\
         \n  plain.css 1:14  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn later() {
            let runner = runner().with_cwd("later");
            assert_eq!(
                runner.err("@import \'plain\';\n"),
                "Error: Expected whitespace.\
         \n  ,\
         \n1 | @media (a) or (b) or(c) {x {y: z}}\
         \n  |                     ^\
         \n  \'\
         \n  plain.css 1:21  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
}
