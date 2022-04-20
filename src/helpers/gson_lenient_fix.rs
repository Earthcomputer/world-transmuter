use std::borrow::Cow;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, tag_no_case, take_until};
use nom::character::complete::{char, space1};
use nom::combinator::{eof, map, opt, recognize, value};
use nom::error::Error;
use nom::{Finish, IResult};
use nom::multi::{many0, many1};
use nom::number::complete::recognize_float;
use nom::sequence::{pair, tuple};

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
    pub fixed_str: Cow<'a, str>,
}

pub fn fix_gson_lenient(input: &str) -> Result<FixedGsonLenient, Error<&str>> {
    let (_, (_, _, (value_type, fixed_str), _, _)) = tuple((
        opt(tag(")]}'\n")),
        space,
        alt((
            map(json_object, |str| (JsonType::Object, str)),
            map(json_array, |str| (JsonType::Array, str)),
            map(keyword, |str| (JsonType::Keyword, str)),
            map(number, |str| (JsonType::Number, str)),
            map(string, |str| (JsonType::String, str)),
            map(unquoted_string, |str| (JsonType::String, str)),
            map(eof, |_| (JsonType::Keyword, Cow::Owned("null".to_owned())))
        )),
        space,
        eof
    ))(input).finish()?;

    Ok(FixedGsonLenient { value_type, fixed_str })
}

fn space(i: &str) -> IResult<&str, Option<Cow<str>>> {
    map(
        many0(alt((
            map(space1, |str| Some(Cow::Borrowed(str))),
            map(is_a("\r\n"), |str| Some(Cow::Borrowed(str))),
            value(None, hash_comment),
            value(None, slash_slash_comment),
            value(None, slash_star_comment)
        ))),
        |vec| {
            // SAFETY: everything is either borrowed from the input or None
            vec.into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) })
        }
    )(i)
}

fn hash_comment(i: &str) -> IResult<&str, ()> {
    value(
        (),
        pair(char('#'), is_not("\n\r"))
    )(i)
}

fn slash_slash_comment(i: &str) -> IResult<&str, ()> {
    value(
        (),
        pair(tag("//"), is_not("\n\r"))
    )(i)
}

fn slash_star_comment(i: &str) -> IResult<&str, ()> {
    value(
        (),
        tuple((
            tag("/*"),
            take_until("*/"),
            tag("*/")
        ))
    )(i)
}

fn number(i: &str) -> IResult<&str, Cow<str>> {
    let parser = |i| recognize_float(i);
    map(
        parser,
        |str| Cow::Borrowed(str)
    )(i)
}

fn string_of_type<'a>(quote: char) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |i| {
        recognize(
            tuple((
                char(quote),
                many0(alt((
                    recognize(tag("\\\\")),
                    recognize(pair(char('\\'), char(quote))),
                    recognize(is_not(['\n', '\r', quote].as_slice()))
                ))),
                char(quote)
            ))
        )(i)
    }
}

fn string(i: &str) -> IResult<&str, Cow<str>> {
    map(
        recognize(
            alt((
                string_of_type('"'),
                string_of_type('\'')
            ))
        ),
        |str| if str.chars().next().unwrap() == '"' {
            Cow::Borrowed(str)
        } else {
            // convert single quotes to double quotes
            Cow::Owned(format!(
                "\"{}\"",
                str.to_owned()
                    .strip_prefix('\'').unwrap()
                    .strip_suffix('\'').unwrap()
                    .replace('"', "\\\"")
            ))
        }
    )(i)
}

fn unquoted_string<'a>(i: &'a str) -> IResult<&'a str, Cow<'a, str>> {
    map(
        recognize(many1(is_not("/\\;#={}[]:, \t\r\n"))),
        |str: &'a str| {
            let mut result = "\"".to_owned();
            for ch in str.chars() {
                if ch == '"' {
                    result.push('\\');
                }
                result.push(ch);
            }
            result.push('"');
            Cow::Owned(result)
        }
    )(i)
}

fn keyword(i: &str) -> IResult<&str, Cow<str>> {
    alt((
        map(
            alt((
                tag("true"),
                tag("false"),
                tag("null"),
            )),
            |str| Cow::Borrowed(str)
        ),
        map(tag_no_case("true"), |_| Cow::Owned("true".to_owned())),
        map(tag_no_case("false"), |_| Cow::Owned("false".to_owned())),
        map(tag_no_case("null"), |_| Cow::Owned("null".to_owned()))
    ))(i)
}

fn json_name(i: &str) -> IResult<&str, Cow<str>> {
    alt((
        string,
        unquoted_string
    ))(i)
}

fn json_name_value(i: &str) -> IResult<&str, Cow<str>> {
    map(
        tuple((
            json_name,
            space,
            alt((
                map(tag(":"), |str| Cow::Borrowed(str)),
                map(tag("=>"), |_| Cow::Owned(":".to_owned())),
                map(char('='), |_| Cow::Owned(":".to_owned()))
            )),
            space,
            json_value,
        )),
        |result| {
            [Some(result.0), result.1, Some(result.2), result.3, Some(result.4)].into_iter()
                .flatten()
                // SAFETY: all borrowed values come from input
                .reduce(|a, b| unsafe { concat_strings(a, b) })
                .unwrap()
        }
    )(i)
}

fn comma(i: &str) -> IResult<&str, Cow<str>> {
    alt((
        map(tag(","), |str| Cow::Borrowed(str)),
        map(tag(";"), |_| Cow::Owned(",".to_owned())),
    ))(i)
}

fn json_object(i: &str) -> IResult<&str, Cow<str>> {
    map(
        tuple((
            tag("{"),
            space,
            alt((
                map(tag("}"), |str| Cow::Borrowed(str)),
                map(
                    tuple((
                        json_name_value,
                        many0(
                            tuple((
                                space,
                                comma,
                                space,
                                json_name_value
                            ))
                        ),
                        space,
                        tag("}")
                    )),
                    |result| {
                        [
                            Some(result.0),
                            result.1.into_iter().map(|elem| {
                                [
                                    elem.0,
                                    Some(elem.1),
                                    elem.2,
                                    Some(elem.3)
                                ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
                            }).reduce(|a, b| unsafe { concat_strings(a, b) }),
                            result.2,
                            Some(Cow::Borrowed(result.3))
                        ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
                    }
                )
            )),
        )),
        |result| {
            [
                Some(Cow::Borrowed(result.0)),
                result.1,
                Some(result.2)
            ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
        }
    )(i)
}

fn json_array_elem(i: &str) -> IResult<&str, Cow<str>> {
    alt((
        json_value,
        map(tag(""), |_| Cow::Owned("null".to_owned()))
    ))(i)
}

fn json_array(i: &str) -> IResult<&str, Cow<str>> {
    map(
        tuple((
            tag("["),
            space,
            alt((
                map(tag("]"), |str| Cow::Borrowed(str)),
                map(
                    tuple((
                        json_array_elem,
                        many0(
                            tuple((
                                space,
                                comma,
                                space,
                                json_array_elem
                            ))
                        ),
                        space,
                        tag("]")
                    )),
                    |result| {
                        [
                            Some(result.0),
                            result.1.into_iter().map(|elem| {
                                [
                                    elem.0,
                                    Some(elem.1),
                                    elem.2,
                                    Some(elem.3)
                                ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
                            }).reduce(|a, b| unsafe { concat_strings(a, b) }),
                            result.2,
                            Some(Cow::Borrowed(result.3))
                        ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
                    }
                )
            ))
        )),
        |result| {
            [
                Some(Cow::Borrowed(result.0)),
                result.1,
                Some(result.2)
            ].into_iter().flatten().reduce(|a, b| unsafe { concat_strings(a, b) }).unwrap()
        }
    )(i)
}

fn json_value(i: &str) -> IResult<&str, Cow<str>> {
    alt((
        keyword,
        number,
        string,
        json_array,
        json_object,
        unquoted_string
    ))(i)
}

// SAFETY: the caller must ensure that if a and b are both borrowed, they must be borrowed from the same original String.
unsafe fn concat_strings<'a>(a: Cow<'a, str>, b: Cow<'a, str>) -> Cow<'a, str> {
    return if let (Cow::Borrowed(a), Cow::Borrowed(b)) = (&a, &b) {
        // SAFETY: this is the invariant checked by the caller
        if str::as_ptr(b).offset_from(str::as_ptr(a)) == a.len() as isize {
            // SAFETY: we know that b lies exactly after a, and they will both live as long as 'a
            Cow::Borrowed(a.get_unchecked(0..a.len() + b.len()))
        } else {
            Cow::Owned(format!("{}{}", a, b))
        }
    } else {
        Cow::Owned(format!("{}{}", &*a, &*b))
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_prefix() {
        let result = fix_gson_lenient(")]}'\n{}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_hash_comment() {
        let result = fix_gson_lenient("{# This is a comment\n\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\n\"foo\":\"bar\"}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_slash_slash_comment() {
        let result = fix_gson_lenient("{// This is a comment\n\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\n\"foo\":\"bar\"}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_slash_star_comment() {
        let result = fix_gson_lenient("{/* This is a comment */\"foo\":\"bar\"}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\":\"bar\"}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_top_level_is_object() {
        let result = fix_gson_lenient("{}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{}", result.as_ref().unwrap().fixed_str);
        assert_eq!(JsonType::Object, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_array() {
        let result = fix_gson_lenient("[]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[]", result.as_ref().unwrap().fixed_str);
        assert_eq!(JsonType::Array, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_string() {
        let result = fix_gson_lenient("\"hello\"");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("\"hello\"", result.as_ref().unwrap().fixed_str);
        assert_eq!(JsonType::String, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_is_number() {
        let result = fix_gson_lenient("+123.45e-67");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("+123.45e-67", result.as_ref().unwrap().fixed_str);
        assert_eq!(JsonType::Number, result.unwrap().value_type);
    }

    #[test]
    fn test_top_level_keywords() {
        for keyword in ["true", "false", "null"] {
            let result = fix_gson_lenient(keyword);
            assert!(result.is_ok(), "{:?}", result);
            assert_eq!(keyword, result.as_ref().unwrap().fixed_str);
            assert_eq!(JsonType::Keyword, result.unwrap().value_type);
        }
    }

    #[test]
    fn test_top_level_unquoted_string() {
        let result = fix_gson_lenient("hello");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("\"hello\"", result.as_ref().unwrap().fixed_str);
        assert_eq!(JsonType::String, result.unwrap().value_type);
    }

    #[test]
    fn test_single_quotes() {
        let result = fix_gson_lenient("{'foo': 'bar'}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\": \"bar\"}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_unquoted_strings() {
        let result = fix_gson_lenient("{foo: bar}");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("{\"foo\": \"bar\"}", result.unwrap().fixed_str);
    }

    #[test]
    fn test_semicolon_in_object() {
        let result = fix_gson_lenient(r#"{"foo": "bar"; "hello": "world"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(r#"{"foo": "bar", "hello": "world"}"#, result.unwrap().fixed_str);
    }

    #[test]
    fn test_equals_in_object() {
        let result = fix_gson_lenient(r#"{"foo" = "bar"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(r#"{"foo" : "bar"}"#, result.unwrap().fixed_str);
    }

    #[test]
    fn test_arrow_in_object() {
        let result = fix_gson_lenient(r#"{"foo" => "bar"}"#);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(r#"{"foo" : "bar"}"#, result.unwrap().fixed_str);
    }

    #[test]
    fn test_semicolon_in_array() {
        let result = fix_gson_lenient("[1;2]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,2]", result.unwrap().fixed_str);
    }

    #[test]
    fn test_implicit_null_middle() {
        let result = fix_gson_lenient("[1,,2]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,null,2]", result.unwrap().fixed_str);
    }

    #[test]
    fn test_implicit_null_start() {
        let result = fix_gson_lenient("[,1]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[null,1]", result.unwrap().fixed_str);
    }

    #[test]
    fn test_implicit_null_end() {
        let result = fix_gson_lenient("[1,]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[1,null]", result.unwrap().fixed_str);
    }

    #[test]
    fn test_implicit_null_start_end() {
        let result = fix_gson_lenient("[,]");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!("[null,null]", result.unwrap().fixed_str);
    }
}
