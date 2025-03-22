//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/lcs.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lcs")
}

mod non_contiguous {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn different_positions() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".s1-1 .c .d .s1-2 .e .s1-3\", \".c .s2-1 .d .e .s2-2 .s2-3\")}\n"
        ),
        "a {\
         \n  b: .s1-1 .c .s2-1 .d .s1-2 .e .s2-2 .s1-3.s2-3;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn same_positions() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".s1-1 .c .d .s1-2 .e .s1-3\", \".s2-1 .c .d .s2-2 .e .s2-3\")}\n"
        ),
        "a {\
         \n  b: .s1-1 .s2-1 .c .d .s1-2 .s2-2 .e .s1-3.s2-3, .s2-1 .s1-1 .c .d .s1-2 .s2-2 .e .s1-3.s2-3, .s1-1 .s2-1 .c .d .s2-2 .s1-2 .e .s1-3.s2-3, .s2-1 .s1-1 .c .d .s2-2 .s1-2 .e .s1-3.s2-3;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // wrong result
fn three_versus_two() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// The longest common subsequence is `.c .d .e`, which is longer than `.f g`, so\
             \n// `.c .d .e` gets unified while `.f .g` gets duplicated.\
             \na {b: selector.unify(\".c .d .e .f .g .s1\", \".f .g .c .d .e .s2\")}\n"
        ),
        "a {\
         \n  b: .f .g .c .d .e .f .g .s1.s2;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn two_versus_one() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// The longest common subsequence is `.c .d`, which is longer than `.e`, so `.c\
             \n// .d` gets unified while `.e` gets duplicated.\
             \na {b: selector.unify(\".c .d .e .s1\", \".e .c .d .s2\")}\n"
        ),
        "a {\
         \n  b: .e .c .d .e .s1.s2;\
         \n}\n"
    );
}
