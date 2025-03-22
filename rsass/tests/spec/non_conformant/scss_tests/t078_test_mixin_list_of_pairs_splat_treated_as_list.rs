//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/078_test_mixin_list_of_pairs_splat_treated_as_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("078_test_mixin_list_of_pairs_splat_treated_as_list")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b, $c) {\
             \n  a: $a;\
             \n  b: $b;\
             \n  c: $c;\
             \n}\n\
             \n.foo {\
             \n  @include foo((a 1, b 2, c 3)...);\
             \n}\n"),
        ".foo {\
         \n  a: a 1;\
         \n  b: b 2;\
         \n  c: c 3;\
         \n}\n"
    );
}
