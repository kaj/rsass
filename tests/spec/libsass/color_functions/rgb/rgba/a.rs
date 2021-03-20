//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/rgb/rgba/a.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  c-1: rgba(0,0,0,-1);\
            \n  c0: rgba(0,0,0,0.0);\
            \n  c1: rgba(0,0,0,0.1);\
            \n  c2: rgba(0,0,0,0.2);\
            \n  c3: rgba(0,0,0,0.3);\
            \n  c4: rgba(0,0,0,0.4);\
            \n  c5: rgba(0,0,0,0.5);\
            \n  c6: rgba(0,0,0,0.6);\
            \n  c7: rgba(0,0,0,0.7);\
            \n  c8: rgba(0,0,0,0.8);\
            \n  c9: rgba(0,0,0,0.9);\
            \n  c10: rgba(0,0,0,1);\
            \n  c11: rgba(0,0,0,1.1);\
            \n  c12: rgba(0,0,0,2);\
            \n}\
            \n\
            \nfoo {\
            \n  c-1: rgba(black,-1);\
            \n  c0: rgba(black,0.0);\
            \n  c1: rgba(black,0.1);\
            \n  c2: rgba(black,0.2);\
            \n  c3: rgba(black,0.3);\
            \n  c4: rgba(black,0.4);\
            \n  c5: rgba(black,0.5);\
            \n  c6: rgba(black,0.6);\
            \n  c7: rgba(black,0.7);\
            \n  c8: rgba(black,0.8);\
            \n  c9: rgba(black,0.9);\
            \n  c10: rgba(black,1);\
            \n  c11: rgba(black,1.1);\
            \n  c12: rgba(black,2);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c-1: rgba(0, 0, 0, 0);\
        \n  c0: rgba(0, 0, 0, 0);\
        \n  c1: rgba(0, 0, 0, 0.1);\
        \n  c2: rgba(0, 0, 0, 0.2);\
        \n  c3: rgba(0, 0, 0, 0.3);\
        \n  c4: rgba(0, 0, 0, 0.4);\
        \n  c5: rgba(0, 0, 0, 0.5);\
        \n  c6: rgba(0, 0, 0, 0.6);\
        \n  c7: rgba(0, 0, 0, 0.7);\
        \n  c8: rgba(0, 0, 0, 0.8);\
        \n  c9: rgba(0, 0, 0, 0.9);\
        \n  c10: black;\
        \n  c11: black;\
        \n  c12: black;\
        \n}\
        \nfoo {\
        \n  c-1: rgba(0, 0, 0, 0);\
        \n  c0: rgba(0, 0, 0, 0);\
        \n  c1: rgba(0, 0, 0, 0.1);\
        \n  c2: rgba(0, 0, 0, 0.2);\
        \n  c3: rgba(0, 0, 0, 0.3);\
        \n  c4: rgba(0, 0, 0, 0.4);\
        \n  c5: rgba(0, 0, 0, 0.5);\
        \n  c6: rgba(0, 0, 0, 0.6);\
        \n  c7: rgba(0, 0, 0, 0.7);\
        \n  c8: rgba(0, 0, 0, 0.8);\
        \n  c9: rgba(0, 0, 0, 0.9);\
        \n  c10: black;\
        \n  c11: black;\
        \n  c12: black;\
        \n}\
        \n"
    );
}
