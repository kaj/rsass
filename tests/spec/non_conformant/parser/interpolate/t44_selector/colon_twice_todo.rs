//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/colon_twice_todo.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
             \n  filter: progid:DXImageTransform.Microsoft.#{Alpha(opacity=20)};\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: expected \"(\".\
         \n  ,\
         \n2 |   filter: progid:DXImageTransform.Microsoft.#{Alpha(opacity=20)};\
         \n  |                                             ^\
         \n  \'\
         \n  input.scss 2:45  root stylesheet",
    );
}
