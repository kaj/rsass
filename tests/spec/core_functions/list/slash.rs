//! Tests auto-converted from "sass-spec/spec/core_functions/list/slash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("slash")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.slash(c)}\n"
            ),
            "Error: At least two elements are required.\
         \n  ,\
         \n2 | a {b: list.slash(c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn many_args() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \na {b: list.slash(c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z)}\n"
        ),
        "a {\
         \n  b: c / d / e / f / g / h / i / j / k / l / m / n / o / p / q / r / s / t / u / v / w / x / y / z;\
         \n}\n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.slash(c, d, e)}\n"),
        "a {\
         \n  b: c / d / e;\
         \n}\n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.slash(c, d)}\n"),
        "a {\
         \n  b: c / d;\
         \n}\n"
    );
}
