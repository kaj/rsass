//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/variable/dimension.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$i1: 10.0001px;\
            \n$j1: 10.01px;\
            \n$k1: -10.0001px;\
            \n$l1: -10.01px;\
            \n\
            \n#foo {\
            \n  i: $i1;\
            \n  j: $j1;\
            \n  k: $k1;\
            \n  l: $l1; }\
            \n\
            \n$i2: 0.0001px;\
            \n$j2: 0.01px;\
            \n$k2: -0.0001px;\
            \n$l2: -0.01px;\
            \n\
            \n#foo {\
            \n  i: $i2;\
            \n  j: $j2;\
            \n  k: $k2;\
            \n  l: $l2; }\
            \n\
            \n$i3: .0001px;\
            \n$j3: .01px;\
            \n$k3: -.0001px;\
            \n$l3: -.01px;\
            \n\
            \n#foo {\
            \n  i: $i3;\
            \n  j: $j3;\
            \n  k: $k3;\
            \n  l: $l3; }\
            \n\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  i: 10.0001px;\
        \n  j: 10.01px;\
        \n  k: -10.0001px;\
        \n  l: -10.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n"
    );
}
