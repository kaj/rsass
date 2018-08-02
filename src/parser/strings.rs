use nom::alphanumeric;
use parser::util::is_name_char;
use parser::value::value_expression;
use sass::{SassString, StringPart};
use std::str::from_utf8;
use value::Quotes;

named!(pub sass_string<SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               map!(selector_string, StringPart::Raw))),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_ext<SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               extended_part)),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_dq<SassString>,
       map!(delimited!(tag!("\""),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(s.to_string())) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\\\"")) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("'")) |
                           extra_escape)),
                       tag!("\"")),
            |p| SassString::new(p, Quotes::Double)));

named!(pub sass_string_sq<SassString>,
       map!(delimited!(tag!("'"),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(s.to_string())) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("\\'")) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\"")) |
                           extra_escape
                               )),
                       tag!("'")),
            |p| SassString::new(p, Quotes::Single)));

named!(
    string_part_interpolation<StringPart>,
    map!(
        delimited!(tag!("#{"), value_expression, tag!("}")),
        StringPart::Interpolation
    )
);

named!(
    simple_qstring_part<StringPart>,
    map!(map_res!(is_not!("\\#'\""), from_utf8), |s| StringPart::Raw(
        s.to_string()
    ))
);

named!(
    selector_string<String>,
    fold_many1!(
        alt_complete!(
            selector_plain_part
                | selector_escaped_part
                | hash_no_interpolation
        ),
        String::new(),
        |mut acc: String, item: &str| {
            acc.push_str(item);
            acc
        }
    )
);
named!(
    selector_plain_part<&str>,
    map_res!(is_not!("\n\t >$\"'\\#+*/()[]{}:;,=!&@"), |s| from_utf8(s))
);
named!(
    selector_escaped_part<&str>,
    map_res!(
        recognize!(preceded!(
            tag!("\\"),
            alt!(value!((), many_m_n!(1, 3, hexpair)) | value!((), take!(1)))
        )),
        |s| from_utf8(s)
    )
);
named!(
    hexpair,
    recognize!(do_parse!(
        one_of!("0123456789ABCDEFabcdef")
            >> one_of!("0123456789ABCDEFabcdef")
            >> ()
    ))
);
named!(
    hash_no_interpolation<&str>,
    map_res!(terminated!(tag!("#"), peek!(not!(tag!("{")))), |s| {
        from_utf8(s)
    })
);
named!(
    extra_escape<StringPart>,
    map!(
        preceded!(
            tag!("\\"),
            alt!(
                alphanumeric
                    | tag!(" ")
                    | tag!("'")
                    | tag!("\"")
                    | tag!("\\")
                    | tag!("#")
            )
        ),
        |s| StringPart::Raw(format!("\\{}", from_utf8(s).unwrap()))
    )
);

named!(pub extended_part<StringPart>,
       map!(recognize!(pair!(take_while1!(is_ext_str_start_char),
                             take_while!(is_ext_str_char))),
            |v| StringPart::Raw(from_utf8(v).unwrap().into())));

fn is_ext_str_start_char(c: u8) -> bool {
    is_name_char(c)
        || c == b'*'
        || c == b'+'
        || c == b'.'
        || c == b'/'
        || c == b':'
        || c == b'='
        || c == b'?'
        || c == b'|'
}
fn is_ext_str_char(c: u8) -> bool {
    is_name_char(c)
        || c == b'*'
        || c == b'+'
        || c == b','
        || c == b'.'
        || c == b'/'
        || c == b':'
        || c == b'='
        || c == b'?'
        || c == b'|'
}
