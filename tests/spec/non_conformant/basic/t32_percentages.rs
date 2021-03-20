//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/32_percentages.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  width: 10% + 20%;\
            \n  height: 10% - 20%;\
            \n  width: 10% + 10;\
            \n  width: 10 + 10%;\
            \n  height: 10% - 10;\
            \n  height: 10 - 10%;\
            \n  blah: (20% / 4%);\
            \n  flah: 12 * 75%;\
            \n  grah: 75% * 12;\
            \n  // hwah: (24 / 8%);\
            \n  nyah: (35% / 7);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 30%;\
        \n  height: -10%;\
        \n  width: 20%;\
        \n  width: 20%;\
        \n  height: 0%;\
        \n  height: 0%;\
        \n  blah: 5;\
        \n  flah: 900%;\
        \n  grah: 900%;\
        \n  nyah: 5%;\
        \n}\
        \n"
    );
}
