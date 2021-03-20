//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_435.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$skin-name: \"CMS_Black\";\r\
            \n\r\
            \n$QUOTE: unquote(\'\"\');\r\
            \n$EMPTY_STRING: unquote( \"\" );\r\
            \n$SLASH: unquote(\"/\");\r\
            \n\r\
            \n$SKINS_PATH: unquote(\"/CMS/Skins\");\r\
            \n$URL_SEPARATOR: $SLASH;\r\
            \n$URL_PREFIX: $EMPTY_STRING;\r\
            \n$URL_SUFFIX: $EMPTY_STRING;\r\
            \n\r\
            \n$_URL_PREFIX: $URL_PREFIX + $EMPTY_STRING;\r\
            \n$_URL_SUFFIX: $URL_SUFFIX + $EMPTY_STRING;\r\
            \n$_URL_SEPARATOR: $URL_SEPARATOR + $EMPTY_STRING;\r\
            \n$_SKINS_PATH: $SKINS_PATH + $EMPTY_STRING;\r\
            \n\r\
            \n@function webresource-image-url( $skin, $control, $file ) \r\
            \n{\r\
            \n\t$_url: $EMPTY_STRING;\r\
            \n\t$_path: $_SKINS_PATH $skin $control;\r\
            \n\r\
            \n\t@each $_part in $_path {\r\
            \n\t\t$_url: $_url + $_part + $_URL_SEPARATOR\r\
            \n\t}\r\
            \n\r\
            \n\t@return $_URL_PREFIX + $QUOTE + $_url + $file + $QUOTE + $_URL_SUFFIX;\r\
            \n}\r\
            \n\r\
            \n@function global-image-url( $skin, $control, $file ) {\r\
            \n\t@return webresource-image-url( $skin, $control, $file );\r\
            \n}\r\
            \n\r\
            \n@function skin-image-url( $control, $file ) {\r\
            \n\t@return global-image-url( $skin-name, $control, $file );\r\
            \n}\r\
            \n\r\
            \n$actions-sprite: skin-image-url( \"Common\", \"radActionsSprite.png\" );\r\
            \n\r\
            \n.test \r\
            \n{\r\
            \n\tbackground-image: url( $actions-sprite );\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  background-image: url(\"/CMS/Skins/CMS_Black/Common/radActionsSprite.png\");\
        \n}\
        \n"
    );
}
