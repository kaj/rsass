//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/098_test_function_list_of_pairs_splat_treated_as_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("098_test_function_list_of_pairs_splat_treated_as_list")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b, $c) {\
             \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
             \n}\n\
             \n.foo {\
             \n  val: foo((a 1, b 2, c 3)...);\
             \n}\n"),
        ".foo {\
         \n  val: \"a: a 1, b: b 2, c: c 3\";\
         \n}\n"
    );
}
