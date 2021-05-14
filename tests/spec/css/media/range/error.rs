//! Tests auto-converted from "sass-spec/spec/css/media/range/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod invalid_binary_operator {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn before_colon() {
        assert_eq!(
        runner().err(
            "// Even though this isn\'t *technically* ambiguous, disallowing it makes parsing\
             \n// much easier because you don\'t have to disambiguate what the first `<` (or\
             \n// other comparison operator) is.\
             \n@media (1 < 2: 10px) {a {b: c}}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n4 | @media (1 < 2: 10px) {a {b: c}}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 4:14  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn eq() {
        assert_eq!(
            runner().err("@media (1 = 2 = width) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1 = 2 = width) {a {b: c}}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn gt() {
        assert_eq!(
            runner().err("@media (3 > width > 2 > 1) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (3 > width > 2 > 1) {a {b: c}}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn gte() {
        assert_eq!(
            runner().err("@media (3 >= width >= 2 >= 1) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (3 >= width >= 2 >= 1) {a {b: c}}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_subexpression() {
        assert_eq!(
        runner().err(
            "// Even though `1 < 2` here isn\'t syntactically at the top-level, because `<`\
             \n// binds more tightly than `or`, it\'s disallowed because it\'s not in parentheses\
             \n// or square brackets.\
             \n@media (1 < 2 or false = width) {a {b: c}}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n4 | @media (1 < 2 or false = width) {a {b: c}}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 4:24  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn lt() {
        assert_eq!(
            runner().err("@media (1 < width < 2 < 3) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1 < width < 2 < 3) {a {b: c}}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lte() {
        assert_eq!(
            runner().err("@media (1 <= width <= 2 <= 3) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1 <= width <= 2 <= 3) {a {b: c}}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
}
mod invalid_comparison {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn gte() {
        assert_eq!(
            runner().err("@media (width > = 100px) {a {b: c}}\n"),
            "Error: Expected expression.\
         \n  ,\
         \n1 | @media (width > = 100px) {a {b: c}}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lte() {
        assert_eq!(
            runner().err("@media (width < = 100px) {a {b: c}}\n"),
            "Error: Expected expression.\
         \n  ,\
         \n1 | @media (width < = 100px) {a {b: c}}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn range_gte() {
        assert_eq!(
            runner().err("@media (10px > width > = 1px) {a {b: c}}\n"),
            "Error: Expected expression.\
         \n  ,\
         \n1 | @media (10px > width > = 1px) {a {b: c}}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
}
mod mismatched_range {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn gt_lt() {
        assert_eq!(
            runner().err("@media (1px > width < 2px) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1px > width < 2px) {a {b: c}}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn gte_lte() {
        assert_eq!(
            runner().err("@media (1px >= width <= 2px) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1px >= width <= 2px) {a {b: c}}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lt_gt() {
        assert_eq!(
            runner().err("@media (1px < width > 2px) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1px < width > 2px) {a {b: c}}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn lte_gte() {
        assert_eq!(
            runner().err("@media (1px <= width >= 2px) {a {b: c}}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | @media (1px <= width >= 2px) {a {b: c}}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
        );
    }
}
