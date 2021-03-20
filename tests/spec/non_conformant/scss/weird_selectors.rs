//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/weird-selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "> > E {\
            \n  color: red;\
            \n}\
            \n\
            \nE > > {\
            \n  color: red;\
            \n}\
            \n\
            \n> > E > > {\
            \n  > > F > > {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "> > E {\
        \n  color: red;\
        \n}\
        \nE > > {\
        \n  color: red;\
        \n}\
        \n> > E > > > > F > > {\
        \n  color: red;\
        \n}\
        \n"
    );
}
