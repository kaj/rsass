//! Tests auto-converted from "sass-spec/spec/css/custom_properties/indentation.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".indentation {\
            \n  --simple: {\
            \n    foo: bar;\
            \n  };\
            \n\
            \n  --empty-line: {\
            \n    foo: bar;\
            \n\
            \n    baz: bang;\
            \n  };\
            \n\
            \n  --multi-level: {\
            \n   one\
            \n    two\
            \n     three\
            \n      four\
            \n  };\
            \n\
            \n  --all-indented: {\
            \n    foo: bar; };\
            \n\
            \n  --below-base:\
            \n    foo\
            \n bar\
            \n   baz;\
            \n\
            \n         --deep-base: {\
            \n           foo: bar;\
            \n         };\
            \n\
            \n\t--hard-tabs: {\
            \n\t\tfoo: bar;\
            \n\t};\
            \n}\
            \n"
        )
        .unwrap(),
        ".indentation {\
        \n  --simple: {\
        \n    foo: bar;\
        \n  };\
        \n  --empty-line: {\
        \n    foo: bar;\
        \n    baz: bang;\
        \n  };\
        \n  --multi-level: {\
        \n   one\
        \n    two\
        \n     three\
        \n      four\
        \n  };\
        \n  --all-indented: {\
        \n    foo: bar; };\
        \n  --below-base:\
        \n     foo\
        \n  bar\
        \n    baz;\
        \n  --deep-base: {\
        \n    foo: bar;\
        \n  };\
        \n  --hard-tabs: {\
        \n  \tfoo: bar;\
        \n  };\
        \n}\
        \n"
    );
}
