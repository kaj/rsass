//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/special_functions/slash_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("slash_list")
}

mod slash_list {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsl(list.slash(1 2% 3%, var(--c)))}\n"),
            "a {\
         \n  b: hsl(1, 2%, 3%, var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn channels() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsl(list.slash(var(--foo), 0.4))}\n"),
            "a {\
         \n  b: hsl(var(--foo) / 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn some_channels() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsl(list.slash(1 var(--foo), 0.4))}\n"),
            "a {\
         \n  b: hsl(1 var(--foo) / 0.4);\
         \n}\n"
        );
    }
}
