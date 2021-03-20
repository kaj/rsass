//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_610.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin vararg-test($a, $b, $c, $d) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a, b, c, d);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a b c d...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a b c d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a, b, c, d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a: a, b: b, c: c, d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((\"a\": a, \"b\": b, \"c\": c, \"d\": d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a b..., (c: c, d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a, b c..., (d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test($c: c, (a: a, b: b, d: d)...);\
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
