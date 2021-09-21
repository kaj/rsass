//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/operator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod greater_than {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
#[ignore] // missing error
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
#[ignore] // missing error
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
