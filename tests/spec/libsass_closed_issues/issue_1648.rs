//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1648.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 3px;\
            \n/* comment 1 */\
            \n@if/* pre 1 */$x == 3px/* post 1 */{\
            \n    /* if 1 */\
            \n}\
            \n/* comment 2 */\
            \n@elseif/* pre 2 */$x == 2px/* post 2 */{\
            \n    /* else if 2 */\
            \n}\
            \n/* comment 3 */\
            \n@else/* middle 3 */if/* pre 3  */$x == 3px/* post 3 */{\
            \n    /* else if 3 */\
            \n}\
            \n/* comment 4 */\
            \n@else/* post 4 */{\
            \n    /* else 4 */\
            \n}\
            \n/* comment 5 */"
        )
        .unwrap(),
        "/* comment 1 */\
        \n/* if 1 */\
        \n/* comment 5 */\
        \n"
    );
}
