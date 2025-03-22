//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/26_selector_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("26_selector_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n$x: oo, ba;\
             \n$y: az, hu;\n\
             \nf#{$x}r {\
             \n  p: 1;\
             \n  b#{$y}x {\
             \n    q: 2;\
             \n    mumble#{list.length($x) + list.length($y)} {\
             \n      r: 3;\
             \n    }\
             \n  }\
             \n}"
        ),
        "foo, bar {\
         \n  p: 1;\
         \n}\
         \nfoo baz, foo hux, bar baz, bar hux {\
         \n  q: 2;\
         \n}\
         \nfoo baz mumble4, foo hux mumble4, bar baz mumble4, bar hux mumble4 {\
         \n  r: 3;\
         \n}\n"
    );
}
