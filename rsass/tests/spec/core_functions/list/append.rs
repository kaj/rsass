//! Tests auto-converted from "sass-spec/spec/core_functions/list/append.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("append")
}

#[test]
fn auto() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.append(c d, e, $separator: auto)}\n"),
        "a {\
         \n  b: c d e;\
         \n}\n"
    );
}
#[test]
fn bracketed() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.append([], 1)}\n"),
        "a {\
         \n  b: [1];\
         \n}\n"
    );
}
mod comma {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn default() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append((1, 2, 3), 4)}\n"),
            "a {\
         \n  b: 1, 2, 3, 4;\
         \n}\n"
        );
    }
    #[test]
    fn overridden() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append(1 2 3, 4, $separator: comma)}\n"),
            "a {\
         \n  b: 1, 2, 3, 4;\
         \n}\n"
        );
    }
}
mod empty {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.append(utils.$empty-comma-list, 1);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: 1;\
         \n  type: list;\
         \n  separator: comma;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.append(utils.$empty-space-list, 1);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: 1;\
         \n  type: list;\
         \n  separator: space;\
         \n}\n"
        );
    }
    #[test]
    fn undecided() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.append((), 1);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: 1;\
         \n  type: list;\
         \n  separator: space;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.append(c)}\n"
            ),
            "Error: Missing argument $val.\
         \n  ,--> input.scss\
         \n2 | a {b: list.append(c)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function append($list, $val, $separator: auto) {\
         \n  |           ===================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.append(c, d, comma, e)}\n"
            ),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: list.append(c, d, comma, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function append($list, $val, $separator: auto) {\
         \n  |           ===================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn separator() {
            assert_eq!(
                runner().err(
                    "@use \"sass:list\";\
             \na {b: list.append(c, d, $separator: 1)}\n"
                ),
                "Error: $separator: 1 is not a string.\
         \n  ,\
         \n2 | a {b: list.append(c, d, $separator: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn unknown_separator() {
        assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.append(c, d, $separator: e)}\n"
        ),
        "Error: $separator: Must be \"space\", \"comma\", \"slash\", or \"auto\".\
         \n  ,\
         \n2 | a {b: list.append(c, d, $separator: e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod map {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.append(utils.$empty-map, 1);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: 1;\
         \n  type: list;\
         \n  separator: space;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append((c: d, e: f), g)}\n"),
            "a {\
         \n  b: c d, e f, g;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.append($list: c d, $val: e, $separator: comma)}\n"),
        "a {\
         \n  b: c, d, e;\
         \n}\n"
    );
}
#[test]
fn non_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.append(c, d)}\n"),
        "a {\
         \n  b: c d;\
         \n}\n"
    );
}
mod single {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append((1,), 2)}\n"),
            "a {\
         \n  b: 1, 2;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.append(utils.with-separator(1, space), 2)}\n"),
            "a {\
         \n  b: 1 2;\
         \n}\n"
        );
    }
    #[test]
    fn undecided() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append(1, 2)}\n"),
            "a {\
         \n  b: 1 2;\
         \n}\n"
        );
    }
}
mod slash {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn default() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append(list.slash(c, d), e)}\n"),
            "a {\
         \n  b: c / d / e;\
         \n}\n"
        );
    }
    #[test]
    fn overridden() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append(c d, e, $separator: slash)}\n"),
            "a {\
         \n  b: c / d / e;\
         \n}\n"
        );
    }
}
mod space {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn default() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append(1 2 3, 4)}\n"),
            "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
        );
    }
    #[test]
    fn overridden() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.append((1, 2, 3), 4, $separator: space)}\n"),
            "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
        );
    }
}
