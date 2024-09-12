//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/relative_color.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("relative_color")
}

mod calc {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner()
                .ok("a {b: color(from #aaa srgb calc(l + 0.2) a b / 25%)}\n"),
            "a {\
         \n  b: color(from #aaa srgb calc(l + 0.2) a b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: color(from #aaa srgb calc(l + 0.2) a b)}\n"),
            "a {\
         \n  b: color(from #aaa srgb calc(l + 0.2) a b);\
         \n}\n"
        );
    }
}
mod different_case {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: color(From #aaa srgb r g b / 25%)}\n"),
            "a {\
         \n  b: color(From #aaa srgb r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: color(From #aaa srgb r g b)}\n"),
            "a {\
         \n  b: color(From #aaa srgb r g b);\
         \n}\n"
        );
    }
}
#[test]
fn slash_list_alpha() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: color(list.slash(from #aaa srgb r g b, 25%))}\n"),
        "a {\
         \n  b: color(from #aaa srgb r g b / 25%);\
         \n}\n"
    );
}
mod test_static {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: color(from #aaa srgb r g b / 25%)}\n"),
            "a {\
         \n  b: color(from #aaa srgb r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: color(from #aaa srgb r g b)}\n"),
            "a {\
         \n  b: color(from #aaa srgb r g b);\
         \n}\n"
        );
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: color(from var(--c) srgb r g b / 25%)}\n"),
            "a {\
         \n  b: color(from var(--c) srgb r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: color(from var(--c) srgb r g b)}\n"),
            "a {\
         \n  b: color(from var(--c) srgb r g b);\
         \n}\n"
        );
    }
}
