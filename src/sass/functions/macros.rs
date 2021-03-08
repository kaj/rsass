macro_rules! name {
    ($name:ident) => {
        crate::sass::Name::from_static(stringify!($name))
    };
    () => {
        // an empty name
        crate::sass::Name::from_static("")
    };
}

macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), None)
    };
    ($name:ident = $value:expr) => {{
        use $crate::parser::parse_value_data;
        (
            stringify!($name).into(),
            Some(parse_value_data($value).expect(stringify!($name))),
        )
    }};
}

macro_rules! def {
    ($f:expr, $name:ident( $($arg:ident$(=$val:expr)* ),* ), $body:expr) => {{
        use crate::sass::{Functions, FormalArgs};
        use std::sync::Arc;
        let args = FormalArgs::new(vec![ $(one_arg!($arg $(=$val)*)),* ]);
        $f.builtin_fn(name!($name), args, Arc::new($body));
    }}
}
macro_rules! def_va {
    ($f:expr, $name:ident( $($arg:ident$(=$val:expr)* ),*), $body:expr) => {{
        use crate::sass::{Functions, FormalArgs};
        use std::sync::Arc;
        let args = FormalArgs::new_va(vec![ $(one_arg!($arg $(=$val)*)),* ]);
        $f.builtin_fn(name!($name), args, Arc::new($body));
    }}
}

macro_rules! dep_warn {
    ($first: expr, $($arg:expr),*) => {{
        use std::sync::Once;
        static WARN: Once = Once::new();
        WARN.call_once(|| {
            eprintln!(concat!("DEPRECATION WARNING: ", $first), $($arg),*);
        });
    }};
    ($first: expr) => {{
        use std::sync::Once;
        static WARN: Once = Once::new();
        WARN.call_once(|| eprintln!(concat!("DEPRECATION WARNING: ", $first)));
    }}
}
