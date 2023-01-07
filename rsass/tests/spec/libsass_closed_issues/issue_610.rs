//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_610.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_610")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin vararg-test($a, $b, $c, $d) {\
             \n  a: $a;\
             \n  b: $b;\
             \n  c: $c;\
             \n  d: $d;\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test(a, b, c, d);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test(a b c d...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test((a b c d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test((a, b, c, d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test((a: a, b: b, c: c, d: d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test((\"a\": a, \"b\": b, \"c\": c, \"d\": d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test(a b..., (c: c, d: d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test(a, b c..., (d: d)...);\
             \n}\n\
             \nfoo {\
             \n  @include vararg-test($c: c, (a: a, b: b, d: d)...);\
             \n}\n"
        ),
        "foo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\
         \nfoo {\
         \n  a: a;\
         \n  b: b;\
         \n  c: c;\
         \n  d: d;\
         \n}\n"
    );
}
