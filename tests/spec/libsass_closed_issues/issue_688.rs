//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_688.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n  /* Convert to px  */\
            \n  px-to-px: 0px + 1px;\
            \n  pt-to-px: 0px + 1pt;\
            \n  pc-to-px: 0px + 1pc;\
            \n  in-to-px: 0px + 1in;\
            \n  mm-to-px: 0px + 1mm;\
            \n  cm-to-px: 0px + 1cm;\
            \n  /* Convert to pt  */\
            \n  px-to-pt: 0pt + 1px;\
            \n  pt-to-pt: 0pt + 1pt;\
            \n  pc-to-pt: 0pt + 1pc;\
            \n  in-to-pt: 0pt + 1in;\
            \n  mm-to-pt: 0pt + 1mm;\
            \n  cm-to-pt: 0pt + 1cm;\
            \n  /* Convert to pc  */\
            \n  px-to-pc: 0pc + 1px;\
            \n  pt-to-pc: 0pc + 1pt;\
            \n  pc-to-pc: 0pc + 1pc;\
            \n  in-to-pc: 0pc + 1in;\
            \n  mm-to-pc: 0pc + 1mm;\
            \n  cm-to-pc: 0pc + 1cm;\
            \n  /* Convert to in  */\
            \n  px-to-in: 0in + 1px;\
            \n  pt-to-in: 0in + 1pt;\
            \n  pc-to-in: 0in + 1pc;\
            \n  in-to-in: 0in + 1in;\
            \n  mm-to-in: 0in + 1mm;\
            \n  cm-to-in: 0in + 1cm;\
            \n  /* Convert to mm  */\
            \n  px-to-mm: 0mm + 1px;\
            \n  pt-to-mm: 0mm + 1pt;\
            \n  pc-to-mm: 0mm + 1pc;\
            \n  in-to-mm: 0mm + 1in;\
            \n  mm-to-mm: 0mm + 1mm;\
            \n  cm-to-mm: 0mm + 1cm; \
            \n  /* Convert to cm  */\
            \n  px-to-cm: 0cm + 1px;\
            \n  pt-to-cm: 0cm + 1pt;\
            \n  pc-to-cm: 0cm + 1pc;\
            \n  in-to-cm: 0cm + 1in;\
            \n  mm-to-cm: 0cm + 1mm;\
            \n  cm-to-cm: 0cm + 1cm;   \
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  /* Convert to px  */\
        \n  px-to-px: 1px;\
        \n  pt-to-px: 1.3333333333px;\
        \n  pc-to-px: 16px;\
        \n  in-to-px: 96px;\
        \n  mm-to-px: 3.7795275591px;\
        \n  cm-to-px: 37.7952755906px;\
        \n  /* Convert to pt  */\
        \n  px-to-pt: 0.75pt;\
        \n  pt-to-pt: 1pt;\
        \n  pc-to-pt: 12pt;\
        \n  in-to-pt: 72pt;\
        \n  mm-to-pt: 2.8346456693pt;\
        \n  cm-to-pt: 28.3464566929pt;\
        \n  /* Convert to pc  */\
        \n  px-to-pc: 0.0625pc;\
        \n  pt-to-pc: 0.0833333333pc;\
        \n  pc-to-pc: 1pc;\
        \n  in-to-pc: 6pc;\
        \n  mm-to-pc: 0.2362204724pc;\
        \n  cm-to-pc: 2.3622047244pc;\
        \n  /* Convert to in  */\
        \n  px-to-in: 0.0104166667in;\
        \n  pt-to-in: 0.0138888889in;\
        \n  pc-to-in: 0.1666666667in;\
        \n  in-to-in: 1in;\
        \n  mm-to-in: 0.0393700787in;\
        \n  cm-to-in: 0.3937007874in;\
        \n  /* Convert to mm  */\
        \n  px-to-mm: 0.2645833333mm;\
        \n  pt-to-mm: 0.3527777778mm;\
        \n  pc-to-mm: 4.2333333333mm;\
        \n  in-to-mm: 25.4mm;\
        \n  mm-to-mm: 1mm;\
        \n  cm-to-mm: 10mm;\
        \n  /* Convert to cm  */\
        \n  px-to-cm: 0.0264583333cm;\
        \n  pt-to-cm: 0.0352777778cm;\
        \n  pc-to-cm: 0.4233333333cm;\
        \n  in-to-cm: 2.54cm;\
        \n  mm-to-cm: 0.1cm;\
        \n  cm-to-cm: 1cm;\
        \n}\
        \n"
    );
}
