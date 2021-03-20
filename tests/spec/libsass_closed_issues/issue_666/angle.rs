//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_666/angle.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n\
            \n  num_deg: 42 + 11deg;\
            \n  num_grad: 42 + 11grad;\
            \n  num_rad: 42 + 11rad;\
            \n  num_turn: 42 + 11turn;\
            \n\
            \n  deg_num: 11deg + 42;\
            \n  deg_grad: 11deg + 42grad;\
            \n  deg_rad: 11deg + 42rad;\
            \n  deg_turn: 11deg + 1turn;\
            \n\
            \n  grad_num: 11grad + 1.5;\
            \n  grad_rad: 11grad + 42rad;\
            \n  grad_turn: 11grad + 0.5turn;\
            \n\
            \n  rad_num: 11rad + 1.5;\
            \n  rad_turn: 11rad + 0.5turn;\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  num_deg: 53deg;\
        \n  num_grad: 53grad;\
        \n  num_rad: 53rad;\
        \n  num_turn: 53turn;\
        \n  deg_num: 53deg;\
        \n  deg_grad: 48.8deg;\
        \n  deg_rad: 2417.4227395495deg;\
        \n  deg_turn: 371deg;\
        \n  grad_num: 12.5grad;\
        \n  grad_rad: 2684.8030439438grad;\
        \n  grad_turn: 211grad;\
        \n  rad_num: 12.5rad;\
        \n  rad_turn: 14.1415926536rad;\
        \n}\
        \n"
    );
}
