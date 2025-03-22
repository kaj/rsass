//! Tests auto-converted from "sass-spec/spec/core_functions/math/variables.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variables")
}

#[test]
fn e() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \n// Multiplied by 1e15 so Sass\'s serialization doesn\'t remove the precision\
             \na {b: math.$e * 1e15}\n"
        ),
        "a {\
         \n  b: 2718281828459045;\
         \n}\n"
    );
}
#[test]
fn epsilon() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \n// Multiplied by 1e31 so Sass\'s serialization doesn\'t remove the precision\
             \na {b: math.$epsilon * 1e31}\n"
        ),
        "a {\
         \n  b: 2220446049250313;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod assignment {
        use super::runner;

        #[test]
        fn e() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$e: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$e: 0;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn epsilon() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$epsilon: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$epsilon: 0;\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn max_number() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$max-number: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$max-number: 0;\
         \n  | ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn max_safe_integer() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$max-safe-integer: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$max-safe-integer: 0;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn min_number() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$min-number: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$min-number: 0;\
         \n  | ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn min_safe_integer() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$min-safe-integer: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$min-safe-integer: 0;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn pi() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \nmath.$pi: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$pi: 0;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
}
#[test]
fn max_number() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \na {b: math.$max-number}\n"
        ),
        "a {\
         \n  b: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;\
         \n}\n"
    );
}
#[test]
fn max_safe_integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.$max-safe-integer}\n"),
        "a {\
         \n  b: 9007199254740991;\
         \n}\n"
    );
}
#[test]
fn min_number() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \n// Multiplied by 1e339 so Sass\'s serialization doesn\'t remove the precision.\
             \n// But 1e339 is too big for a double, so we multiply it multiple times.\
             \na {b: math.$min-number * 1e300 * 1e39}\n"
        ),
        "a {\
         \n  b: 4940656458412465;\
         \n}\n"
    );
}
#[test]
fn min_safe_integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.$min-safe-integer}\n"),
        "a {\
         \n  b: -9007199254740991;\
         \n}\n"
    );
}
#[test]
fn pi() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \n// Multiplied by 1e15 so Sass\'s serialization doesn\'t remove the precision\
             \na {b: math.$pi * 1e15}\n"
        ),
        "a {\
         \n  b: 3141592653589793;\
         \n}\n"
    );
}
