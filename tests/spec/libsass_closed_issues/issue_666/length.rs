//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_666/length.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n\
            \n  num_in: 42 + 11in;\
            \n  num_cm: 42 + 11cm;\
            \n  num_pc: 42 + 11px;\
            \n  num_mm: 42 + 11mm;\
            \n  num_pt: 42 + 11pt;\
            \n  num_px: 42 + 11px;\
            \n\
            \n  in_num: 11in + 42;\
            \n  in_cm: 11in + 42cm;\
            \n  in_pc: 11in + 42pc;\
            \n  in_mm: 11in + 42mm;\
            \n  in_pt: 11in + 42pt;\
            \n  in_px: 11in + 42px;\
            \n\
            \n  cm_num: 11cm + 42;\
            \n  cm_pc: 11cm + 42pc;\
            \n  cm_mm: 11cm + 42mm;\
            \n  cm_pt: 11cm + 42pt;\
            \n  cm_px: 11cm + 42px;\
            \n\
            \n  pc_num: 11pc + 42;\
            \n  pc_mm: 11pc + 42mm;\
            \n  pc_pt: 11pc + 42pt;\
            \n  pc_px: 11pc + 42px;\
            \n\
            \n  mm_num: 11mm + 42;\
            \n  mm_pt: 11mm + 42pt;\
            \n  mm_px: 11mm + 42px;\
            \n\
            \n  pt_num: 11pt + 42;\
            \n  pt_px: 11pt + 42px;\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  num_in: 53in;\
        \n  num_cm: 53cm;\
        \n  num_pc: 53px;\
        \n  num_mm: 53mm;\
        \n  num_pt: 53pt;\
        \n  num_px: 53px;\
        \n  in_num: 53in;\
        \n  in_cm: 27.5354330709in;\
        \n  in_pc: 18in;\
        \n  in_mm: 12.6535433071in;\
        \n  in_pt: 11.5833333333in;\
        \n  in_px: 11.4375in;\
        \n  cm_num: 53cm;\
        \n  cm_pc: 28.78cm;\
        \n  cm_mm: 15.2cm;\
        \n  cm_pt: 12.4816666667cm;\
        \n  cm_px: 12.11125cm;\
        \n  pc_num: 53pc;\
        \n  pc_mm: 20.9212598425pc;\
        \n  pc_pt: 14.5pc;\
        \n  pc_px: 13.625pc;\
        \n  mm_num: 53mm;\
        \n  mm_pt: 25.8166666667mm;\
        \n  mm_px: 22.1125mm;\
        \n  pt_num: 53pt;\
        \n  pt_px: 42.5pt;\
        \n}\
        \n"
    );
}
