//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/operator.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("operator")
}

mod greater_than {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) > calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) > calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) > calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) > 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) > 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) > 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 > calc(var(--c))}\n"),
            "Error: Undefined operation \"1 > calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 > calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod greater_than_or_equals {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
        runner().err(
            "a {b: calc(var(--c)) >= calc(var(--d))}\n"
        ),
        "Error: Undefined operation \"calc(var(--c)) >= calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) >= calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) >= 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) >= 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) >= 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 >= calc(var(--c))}\n"),
            "Error: Undefined operation \"1 >= calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 >= calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod less_than {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) < calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) < calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) < calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) < 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) < 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) < 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 < calc(var(--c))}\n"),
            "Error: Undefined operation \"1 < calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 < calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod less_than_or_equals {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
        runner().err(
            "a {b: calc(var(--c)) <= calc(var(--d))}\n"
        ),
        "Error: Undefined operation \"calc(var(--c)) <= calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) <= calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) <= 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) <= 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) <= 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 <= calc(var(--c))}\n"),
            "Error: Undefined operation \"1 <= calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 <= calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod minus {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) - calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) - calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) - calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) - 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) - 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) - 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 - calc(var(--c))}\n"),
            "Error: Undefined operation \"1 - calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 - calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod test_mod {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) % calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) % calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) % calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) % 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) % 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) % 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 % calc(var(--c))}\n"),
            "Error: Undefined operation \"1 % calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 % calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod plus {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) + calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) + calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) + calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) + 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) + 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) + 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 + calc(var(--c))}\n"),
            "Error: Undefined operation \"1 + calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 + calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod times {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) * calc(var(--d))}\n"),
            "Error: Undefined operation \"calc(var(--c)) * calc(var(--d))\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) * calc(var(--d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lhs() {
        assert_eq!(
            runner().err("a {b: calc(var(--c)) * 1}\n"),
            "Error: Undefined operation \"calc(var(--c)) * 1\".\
         \n  ,\
         \n1 | a {b: calc(var(--c)) * 1}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().err("a {b: 1 * calc(var(--c))}\n"),
            "Error: Undefined operation \"1 * calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: 1 * calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn unary_minus() {
    assert_eq!(
        runner().err("a {b: -(calc(var(--c)))}\n"),
        "Error: Undefined operation \"-calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: -(calc(var(--c)))}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn unary_plus() {
    assert_eq!(
        runner().err("a {b: +calc(var(--c))}\n"),
        "Error: Undefined operation \"+calc(var(--c))\".\
         \n  ,\
         \n1 | a {b: +calc(var(--c))}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
