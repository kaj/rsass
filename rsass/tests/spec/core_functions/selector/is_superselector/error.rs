//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod sub {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn invalid() {
        assert_eq!(
            runner().err("a {b: is-superselector(\"c\", \"[d\")}\n"),
            "Error: $sub: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: is-superselector(\"c\", \"[d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().err("a {b: is-superselector(\"c\", \"&\")}\n"),
            "Error: $sub: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: is-superselector(\"c\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: is-superselector(\"c\", 1)}\n"),
            "Error: $sub: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: is-superselector(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod test_super {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn invalid() {
        assert_eq!(
            runner().err("a {b: is-superselector(\"[c\", \"d\")}\n"),
            "Error: $super: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: is-superselector(\"[c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().err("a {b: is-superselector(\"&\", \"c\")}\n"),
            "Error: $super: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: is-superselector(\"&\", \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: is-superselector(1, \"c\")}\n"),
            "Error: $super: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: is-superselector(1, \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: is-superselector(\"c\")}\n"),
        "Error: Missing argument $sub.\
         \n  ,--> input.scss\
         \n1 | a {b: is-superselector(\"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function is-superselector($super, $sub) {\
         \n  |           ============================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: is-superselector(\"c\", \"d\", \"e\")}\n"),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: is-superselector(\"c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function is-superselector($super, $sub) {\
         \n  |           ============================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
