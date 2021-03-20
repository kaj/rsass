//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/variable/percent.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$e1: 10.0001%;\
            \n$f1: 10.01%;\
            \n$g1: -10.0001%;\
            \n$h1: -10.01%;\
            \n\
            \n#foo {\
            \n  e: $e1;\
            \n  f: $f1;\
            \n  g: $g1;\
            \n  h: $h1; }\
            \n$e2: 0.0001%;\
            \n$f2: 0.01%;\
            \n$g2: -0.0001%;\
            \n$h2: -0.01%;\
            \n\
            \n#foo {\
            \n  e: $e2;\
            \n  f: $f2;\
            \n  g: $g2;\
            \n  h: $h2; }\
            \n\
            \n$e3: .0001%;\
            \n$f3: .01%;\
            \n$g3: -.0001%;\
            \n$h3: -.01%;\
            \n\
            \n#foo {\
            \n  e: $e3;\
            \n  f: $f3;\
            \n  g: $g3;\
            \n  h: $h3; }\
            \n\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  e: 10.0001%;\
        \n  f: 10.01%;\
        \n  g: -10.0001%;\
        \n  h: -10.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n"
    );
}
