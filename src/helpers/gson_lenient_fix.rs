use java_string::{JavaStr, JavaString};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, tag_no_case, take_until};
use nom::combinator::{eof, map, opt, recognize, value};
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::number::complete::recognize_float;
use nom::sequence::{pair, tuple};
use nom::{Finish, IResult};
use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum JsonType {
    Object,
    Array,
    String,
    Number,
    Keyword,
}

#[derive(Clone, Debug)]
pub struct FixedGsonLenient<'a> {
    pub value_type: JsonType,
    pub fixed_str: Cow<'a, JavaStr>,
}

pub fn fix_gson_lenient(input: &JavaStr) -> Result<FixedGsonLenient, Error<&[u8]>> {
    let (_, (_, _, (value_type, fixed_str), _, _)) = tuple((
        opt(tag(b")]}'\n")),
        space,
        alt((
            map(json_object, |str| (JsonType::Object, str)),
            map(json_array, |str| (JsonType::Array, str)),
            map(keyword, |str| (JsonType::Keyword, str)),
            map(number, |str| (JsonType::Number, str)),
            map(string, |str| (JsonType::String, str)),
            map(unquoted_string, |str| (JsonType::String, str)),
            map(eof, |_| (JsonType::Keyword, Cow::Owned(b"null".to_vec()))),
        )),
        space,
        eof,
    ))(input.as_bytes())
    .finish()?;

    Ok(FixedGsonLenient {
        value_type,
        fixed_str: match fixed_str {
            // SAFETY: we only touched the ASCII parts of the string, so we should be safe to convert back to semi-UTF-8
            Cow::Borrowed(str) => unsafe { Cow::Borrowed(JavaStr::from_semi_utf8_unchecked(str)) },
            Cow::Owned(str) => unsafe { Cow::Owned(JavaString::from_semi_utf8_unchecked(str)) },
        },
    })
}

fn space(i: &[u8]) -> IResult<&[u8], Option<Cow<[u8]>>> {
    map(
        many0(alt((
            map(is_a(&b" \t\r\n"[..]), |str| Some(Cow::Borrowed(str))),
            value(None, hash_comment),
            value(None, slash_slash_comment),
            value(None, slash_star_comment),
        ))),
        |vec| {
            // SAFETY: everything is either borrowed from the input or None
            vec.into_iter()
                .flatten()
                .reduce(|a, b| unsafe { concat_strings(a, b) })
        },
    )(i)
}

fn hash_comment(i: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(tag(b"#"), is_not(&b"\n\r"[..])))(i)
}

fn slash_slash_comment(i: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(tag(b"//"), is_not(&b"\n\r"[..])))(i)
}

fn slash_star_comment(i: &[u8]) -> IResult<&[u8], ()> {
    value((), tuple((tag(b"/*"), take_until(&b"*/"[..]), tag(b"*/"))))(i)
}

fn number(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    map(recognize_float, Cow::Borrowed)(i)
}

fn string_of_type<'a>(quote: u8) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |i| {
        recognize(tuple((
            tag(&[quote]),
            many0(alt((
                recognize(tag(b"\\\\")),
                recognize(pair(tag(b"\\"), tag(&[quote]))),
                recognize(is_not([b'\n', b'\r', quote].as_slice())),
            ))),
            tag(&[quote]),
        )))(i)
    }
}

fn string(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    map(
        recognize(alt((string_of_type(b'"'), string_of_type(b'\'')))),
        |str| {
            if str.starts_with(b"\"") {
                Cow::Borrowed(str)
            } else {
                // convert single quotes to double quotes
                let mut result = vec![b'"'];
                for ch in &str[1..str.len() - 1] {
                    if *ch == b'"' {
                        result.push(b'\\');
                    }
                    result.push(*ch);
                }
                result.push(b'"');
                Cow::Owned(result)
            }
        },
    )(i)
}

fn unquoted_string<'a>(i: &'a [u8]) -> IResult<&'a [u8], Cow<'a, [u8]>> {
    map(
        recognize(many1(is_not(&b"/\\;#={}[]:, \t\r\n"[..]))),
        |str: &'a [u8]| {
            let mut result = vec![b'"'];
            for ch in str {
                if *ch == b'"' {
                    result.push(b'\\');
                }
                result.push(*ch);
            }
            result.push(b'"');
            Cow::Owned(result)
        },
    )(i)
}

fn keyword(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    alt((
        map(
            alt((tag(b"true"), tag(b"false"), tag(b"null"))),
            Cow::Borrowed,
        ),
        map(tag_no_case(b"true"), |_| Cow::Owned(b"true".to_vec())),
        map(tag_no_case(b"false"), |_| Cow::Owned(b"false".to_vec())),
        map(tag_no_case(b"null"), |_| Cow::Owned(b"null".to_vec())),
    ))(i)
}

fn json_name(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    alt((string, unquoted_string))(i)
}

fn json_name_value(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    map(
        tuple((
            json_name,
            space,
            alt((
                map(tag(b":"), Cow::Borrowed),
                map(tag(b"=>"), |_| Cow::Owned(vec![b':'])),
                map(tag(b"="), |_| Cow::Owned(vec![b':'])),
            )),
            space,
            json_value,
        )),
        |result| {
            [
                Some(result.0),
                result.1,
                Some(result.2),
                result.3,
                Some(result.4),
            ]
            .into_iter()
            .flatten()
            // SAFETY: all borrowed values come from input
            .reduce(|a, b| unsafe { concat_strings(a, b) })
            .unwrap()
        },
    )(i)
}

fn comma(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    alt((
        map(tag(b","), Cow::Borrowed),
        map(tag(b";"), |_| Cow::Owned(vec![b','])),
    ))(i)
}

fn json_object(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    map(
        tuple((
            tag(b"{"),
            space,
            alt((
                map(tag(b"}"), Cow::Borrowed),
                map(
                    tuple((
                        json_name_value,
                        many0(tuple((space, comma, space, json_name_value))),
                        space,
                        tag(b"}"),
                    )),
                    |result| {
                        [
                            Some(result.0),
                            result
                                .1
                                .into_iter()
                                .map(|elem| {
                                    [elem.0, Some(elem.1), elem.2, Some(elem.3)]
                                        .into_iter()
                                        .flatten()
                                        .reduce(|a, b| unsafe { concat_strings(a, b) })
                                        .unwrap()
                                })
                                .reduce(|a, b| unsafe { concat_strings(a, b) }),
                            result.2,
                            Some(Cow::Borrowed(result.3)),
                        ]
                        .into_iter()
                        .flatten()
                        .reduce(|a, b| unsafe { concat_strings(a, b) })
                        .unwrap()
                    },
                ),
            )),
        )),
        |result| {
            [Some(Cow::Borrowed(result.0)), result.1, Some(result.2)]
                .into_iter()
                .flatten()
                .reduce(|a, b| unsafe { concat_strings(a, b) })
                .unwrap()
        },
    )(i)
}

fn json_array_elem(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    alt((json_value, map(tag(b""), |_| Cow::Owned(b"null".to_vec()))))(i)
}

fn json_array(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    map(
        tuple((
            tag(b"["),
            space,
            alt((
                map(tag(b"]"), Cow::Borrowed),
                map(
                    tuple((
                        json_array_elem,
                        many0(tuple((space, comma, space, json_array_elem))),
                        space,
                        tag("]"),
                    )),
                    |result| {
                        [
                            Some(result.0),
                            result
                                .1
                                .into_iter()
                                .map(|elem| {
                                    [elem.0, Some(elem.1), elem.2, Some(elem.3)]
                                        .into_iter()
                                        .flatten()
                                        .reduce(|a, b| unsafe { concat_strings(a, b) })
                                        .unwrap()
                                })
                                .reduce(|a, b| unsafe { concat_strings(a, b) }),
                            result.2,
                            Some(Cow::Borrowed(result.3)),
                        ]
                        .into_iter()
                        .flatten()
                        .reduce(|a, b| unsafe { concat_strings(a, b) })
                        .unwrap()
                    },
                ),
            )),
        )),
        |result| {
            [Some(Cow::Borrowed(result.0)), result.1, Some(result.2)]
                .into_iter()
                .flatten()
                .reduce(|a, b| unsafe { concat_strings(a, b) })
                .unwrap()
        },
    )(i)
}

fn json_value(i: &[u8]) -> IResult<&[u8], Cow<[u8]>> {
    alt((
        keyword,
        number,
        string,
        json_array,
        json_object,
        unquoted_string,
    ))(i)
}

// SAFETY: the caller must ensure that if a and b are both borrowed, they must be borrowed from the same original String.
unsafe fn concat_strings<'a>(a: Cow<'a, [u8]>, b: Cow<'a, [u8]>) -> Cow<'a, [u8]> {
    if let (Cow::Borrowed(a), Cow::Borrowed(b)) = (&a, &b) {
        // SAFETY: this is the invariant checked by the caller
        if (*b).as_ptr().offset_from((*a).as_ptr()) == a.len() as isize {
            // SAFETY: we know that b lies exactly after a, and they will both live as long as 'a
            Cow::Borrowed(a.get_unchecked(0..a.len() + b.len()))
        } else {
            let mut result = a.to_vec();
            result.extend_from_slice(b);
            Cow::Owned(result)
        }
    } else {
        let mut result = a.into_owned();
        result.extend_from_slice(&b);
        Cow::Owned(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fix_gson_lenient(input: &str) -> Result<FixedGsonLenient, Error<&[u8]>> {
        super::fix_gson_lenient(JavaStr::from_str(input))
    }

    #[test]
    fn test_exec_prefix() {
        let result = fix_gson_lenient(")]}'\n{}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_hash_comment() {
        let result = fix_gson_lenient("{# This is a comment\n\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\n\"foo\":\"bar\"}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_slash_slash_comment() {
        let result = fix_gson_lenient("{// This is a comment\n\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\n\"foo\":\"bar\"}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_slash_star_comment() {
        let result = fix_gson_lenient("{/* This is a comment */\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\":\"bar\"}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_top_level_is_object() {
        let result = fix_gson_lenient("{}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{}", result.as_ref().unwrap().fixed_str.as_ref());
        assert_eq!(JsonType::Object, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_array() {
        let result = fix_gson_lenient("[]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[]", result.as_ref().unwrap().fixed_str.as_ref());
        assert_eq!(JsonType::Array, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_string() {
        let result = fix_gson_lenient("\"hello\"");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("\"hello\"", result.as_ref().unwrap().fixed_str.as_ref());
        assert_eq!(JsonType::String, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_number() {
        let result = fix_gson_lenient("+123.45e-67");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("+123.45e-67", result.as_ref().unwrap().fixed_str.as_ref());
        assert_eq!(JsonType::Number, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_keywords() {
        for keyword in ["true", "false", "null"] {
            let result = fix_gson_lenient(keyword);
            assert!(result.is_ok(), "{:?}", result);
            assert_eq!(keyword, result.as_ref().unwrap().fixed_str.as_ref());
            assert_eq!(JsonType::Keyword, result.unwrap().value_type);
        }
    }

    #[test]
    fn test_top_level_unquoted_string() {
        let result = fix_gson_lenient("hello");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("\"hello\"", result.as_ref().unwrap().fixed_str.as_ref());
        assert_eq!(JsonType::String, result.unwrap().value_type);
    }

    #[test]
    fn test_single_quotes() {
        let result = fix_gson_lenient("{'foo': 'bar'}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\": \"bar\"}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_unquoted_strings() {
        let result = fix_gson_lenient("{foo: bar}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\": \"bar\"}", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_semicolon_in_object() {
        let result = fix_gson_lenient(r#"{"foo": "bar"; "hello": "world"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(
            r#"{"foo": "bar", "hello": "world"}"#,
            result.unwrap().fixed_str.as_ref()
        );
    }

    #[test]
    fn test_equals_in_object() {
        let result = fix_gson_lenient(r#"{"foo" = "bar"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(r#"{"foo" : "bar"}"#, result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_arrow_in_object() {
        let result = fix_gson_lenient(r#"{"foo" => "bar"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(r#"{"foo" : "bar"}"#, result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_semicolon_in_array() {
        let result = fix_gson_lenient("[1;2]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,2]", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_implicit_null_middle() {
        let result = fix_gson_lenient("[1,,2]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,null,2]", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_implicit_null_start() {
        let result = fix_gson_lenient("[,1]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[null,1]", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_implicit_null_end() {
        let result = fix_gson_lenient("[1,]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,null]", result.unwrap().fixed_str.as_ref());
    }

    #[test]
    fn test_implicit_null_start_end() {
        let result = fix_gson_lenient("[,]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[null,null]", result.unwrap().fixed_str.as_ref());
    }
}
