use java_string::{JavaCodePoint, JavaStr, JavaString};
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::combinator::{map, map_res, value};
use nom::error::Error;
use nom::multi::{many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::{AsChar, Finish, IResult};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use world_transmuter_engine::{JCompound, JList, JValue, JValueRef};

#[derive(Debug, PartialEq)]
pub struct ParseError(Error<String>);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Indent(u32);

impl Indent {
    fn indent(&mut self) {
        self.0 += 1;
    }

    fn dedent(&mut self) {
        self.0 -= 1;
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            f.write_str("  ")?;
        }
        Ok(())
    }
}

/// `round_trip` corresponds to the same argument from [parse_compound].
pub fn stringify_compound(map: JCompound, round_trip: bool, pretty: bool) -> JavaString {
    let mut str = JavaString::new();
    stringify(
        JValueRef::Compound(&map),
        &mut str,
        round_trip,
        pretty,
        Indent(0),
    )
    .expect("Should not get Err writing to String");
    str
}

fn stringify(
    obj: JValueRef,
    str: &mut JavaString,
    round_trip: bool,
    pretty: bool,
    mut indent: Indent,
) -> std::fmt::Result {
    match obj {
        JValueRef::Byte(b) => write!(str, "{}", b)?,
        JValueRef::Short(s) => write!(str, "{}", s)?,
        JValueRef::Int(i) => write!(str, "{}", i)?,
        JValueRef::Long(l) => write!(str, "{}", l)?,
        JValueRef::Float(f) => write!(str, "{}", f)?,
        JValueRef::Double(d) => write!(str, "{}", d)?,
        JValueRef::ByteArray(arr) => {
            if round_trip && arr.len() == 1 && arr[0] <= 2 {
                match arr[0] {
                    0 => str.push_str("false"),
                    1 => str.push_str("true"),
                    2 => str.push_str("null"),
                    _ => unreachable!(),
                }
            } else {
                str.push('[');
                if pretty {
                    indent.indent();
                    write!(str, "\n{indent}")?;
                }
                for (i, &b) in <&[i8]>::into_iter(arr).enumerate() {
                    if i != 0 {
                        str.push(',');
                        if pretty {
                            write!(str, "\n{indent}")?;
                        }
                    }
                    write!(str, "{}", b)?;
                }
                if pretty {
                    indent.dedent();
                    write!(str, "\n{indent}")?;
                }
                str.push(']');
            }
        }
        JValueRef::IntArray(arr) => {
            str.push('[');
            if pretty {
                indent.indent();
                write!(str, "\n{indent}")?;
            }
            for (i, &int) in <&[i32]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                    if pretty {
                        write!(str, "\n{indent}")?;
                    }
                }
                write!(str, "{}", int)?;
            }
            if pretty {
                indent.dedent();
                write!(str, "\n{indent}")?;
            }
            str.push(']');
        }
        JValueRef::LongArray(arr) => {
            str.push('[');
            if pretty {
                indent.indent();
                write!(str, "\n{indent}")?;
            }
            for (i, &l) in <&[i64]>::into_iter(arr).enumerate() {
                if i != 0 {
                    str.push(',');
                    if pretty {
                        write!(str, "\n{indent}")?;
                    }
                }
                write!(str, "{}", l)?;
            }
            if pretty {
                indent.dedent();
                write!(str, "\n{indent}")?;
            }
            str.push(']');
        }
        JValueRef::List(list) => {
            str.push('[');
            if pretty {
                indent.indent();
                write!(str, "\n{indent}")?;
            }
            for (i, obj) in list.iter().enumerate() {
                if i != 0 {
                    str.push(',');
                    if pretty {
                        write!(str, "\n{indent}")?;
                    }
                }
                stringify(obj, str, round_trip, pretty, indent)?;
            }
            if pretty {
                indent.dedent();
                write!(str, "\n{indent}")?;
            }
            str.push(']');
        }
        JValueRef::Compound(map) => {
            str.push('{');
            if pretty {
                indent.indent();
                write!(str, "\n{indent}")?;
            }
            for (i, (key, value)) in map.iter().enumerate() {
                if i != 0 {
                    str.push(',');
                    if pretty {
                        write!(str, "\n{indent}")?;
                    }
                }
                stringify_string(key, str);
                str.push(':');
                if pretty {
                    str.push(' ');
                }
                stringify(value.as_value_ref(), str, round_trip, pretty, indent)?;
            }
            if pretty {
                indent.dedent();
                write!(str, "\n{indent}")?;
            }
            str.push('}');
        }
        JValueRef::String(input) => stringify_string(input, str),
    }
    Ok(())
}

fn stringify_string(input: &JavaStr, output: &mut JavaString) {
    const QUOTE: u32 = b'"' as u32;
    const BACKSLASH: u32 = b'\\' as u32;
    const NEWLINE: u32 = b'\n' as u32;
    const CARRIAGE_RETURN: u32 = b'\r' as u32;
    const TAB: u32 = b'\t' as u32;

    output.push('"');
    for ch in input.chars() {
        match ch.as_u32() {
            QUOTE => output.push_str("\\\""),
            BACKSLASH => output.push_str("\\\\"),
            NEWLINE => output.push_str("\\n"),
            CARRIAGE_RETURN => output.push_str("\\r"),
            TAB => output.push_str("\\t"),
            _ => output.push_java(ch),
        }
    }
    output.push('"');
}

/// If `round_trip` is true, encodes `false`, `true` and `null` as `[0]`, `[1]` and `[2]` byte arrays respectively.
pub fn parse_compound(json: &JavaStr, round_trip: bool) -> Result<JCompound, ParseError> {
    preceded(space, |i| object(i, round_trip))(json.as_bytes())
        .finish()
        .map(|(_, o)| o)
        .map_err(|err| {
            ParseError(Error {
                input: String::from_utf8_lossy(err.input).into_owned(),
                code: err.code,
            })
        })
}

fn space(i: &[u8]) -> IResult<&[u8], ()> {
    value((), many0(is_a(&b" \t\r\n"[..])))(i)
}

fn any(i: &[u8], round_trip: bool) -> IResult<&[u8], JValue> {
    alt((
        map(|i| object(i, round_trip), JValue::Compound),
        map(|i| array(i, round_trip), JValue::List),
        map(string, JValue::String),
        map_res(terminated(recognize_float, space), |str| {
            let str = unsafe { JavaStr::from_semi_utf8_unchecked(str) };
            Result::<_, java_string::ParseError<<f64 as FromStr>::Err>>::Ok(
                match JavaStr::parse::<i64>(str) {
                    Ok(long) => JValue::Long(long),
                    Err(_) => JValue::Double(JavaStr::parse::<f64>(str)?),
                },
            )
        }),
        map(pair(tag(b"false"), space), |_| {
            if round_trip {
                JValue::ByteArray(vec![0])
            } else {
                JValue::Byte(0)
            }
        }),
        map(pair(tag(b"true"), space), |_| {
            if round_trip {
                JValue::ByteArray(vec![1])
            } else {
                JValue::Byte(1)
            }
        }),
        map(pair(tag(b"null"), space), |_| {
            if round_trip {
                JValue::ByteArray(vec![2])
            } else {
                JValue::Byte(0)
            }
        }),
    ))(i)
}

fn object(i: &[u8], round_trip: bool) -> IResult<&[u8], JCompound> {
    map(
        delimited(
            pair(tag(b"{"), space),
            separated_list0(
                pair(tag(b","), space),
                separated_pair(string, pair(tag(b":"), space), |i| any(i, round_trip)),
            ),
            pair(tag(b"}"), space),
        ),
        |vec| {
            let mut map = JCompound::new();
            for (k, v) in vec {
                map.insert(k, v);
            }
            map
        },
    )(i)
}

fn array(i: &[u8], round_trip: bool) -> IResult<&[u8], JList> {
    map_res(
        delimited(
            pair(tag(b"["), space),
            separated_list0(pair(tag(b","), space), |i| any(i, round_trip)),
            pair(tag(b"]"), space),
        ),
        |vec| {
            let mut list = JList::new();
            for v in vec {
                if !list.try_push(v) {
                    return Err(());
                }
            }
            Ok(list)
        },
    )(i)
}

fn string(i: &[u8]) -> IResult<&[u8], JavaString> {
    delimited(tag(b"\""), escape_transform, pair(tag(b"\""), space))(i)
}

fn escape_transform(mut i: &[u8]) -> IResult<&[u8], JavaString> {
    let mut result = Vec::new();
    while !i.is_empty() {
        let ch = i[0];

        if ch == b'"' || ch == b'\n' || ch == b'\r' {
            break;
        }

        if ch == b'\\' {
            if i.len() < 2 {
                break;
            }
            let ch2 = i[1];
            match ch2 {
                b'\\' => result.push(b'\\'),
                b'"' => result.push(b'"'),
                b'\'' => result.push(b'\''),
                b'n' => result.push(b'\n'),
                b'r' => result.push(b'\r'),
                b't' => result.push(b'\t'),
                b'u' => {
                    if i.len() < 6 {
                        break;
                    }
                    if !i[2..6].iter().all(|ch| ch.is_hex_digit()) {
                        break;
                    }
                    // SAFETY: we just checked that all bytes in this range are hex digits, so valid UTF-8
                    let Ok(ch) =
                        u32::from_str_radix(unsafe { std::str::from_utf8_unchecked(&i[2..6]) }, 16)
                    else {
                        break;
                    };
                    let Some(ch) = JavaCodePoint::from_u32(ch) else {
                        break;
                    };
                    result.extend_from_slice(ch.encode_semi_utf8(&mut [0; 4]));
                    i = &i[6..];
                    continue;
                }
                _ => break,
            }
            i = &i[2..];
        } else {
            i = &i[1..];
            result.push(ch);
        }
    }
    // SAFETY: we only made ASCII changes, and terminate the loop on ASCII characters which are char boundaries.
    Ok((i, unsafe { JavaString::from_semi_utf8_unchecked(result) }))
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use java_string::JavaStr;
    use valence_nbt::Value;
    use world_transmuter_engine::{compound_to_java, JCompound};

    fn parse_compound(json: &str) -> Result<JCompound, ParseError> {
        super::parse_compound(JavaStr::from_str(json), false)
    }

    fn from_snbt_str(snbt: &str) -> JCompound {
        let Value::Compound(compound) = valence_nbt::snbt::from_snbt_str(snbt).unwrap() else {
            unreachable!("input a non-compound value");
        };
        compound_to_java(compound)
    }

    #[test]
    fn test_parse_object() {
        assert_eq!(
            parse_compound(r#"{"foo": "bar", "baz": "quux"}"#).unwrap(),
            from_snbt_str(r#"{"foo": "bar", "baz": "quux"}"#)
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            parse_compound(r#"{"foo": ["bar", "baz", "quux"]}"#).unwrap(),
            from_snbt_str(r#"{"foo": ["bar", "baz", "quux"]}"#)
        )
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(
            parse_compound(r#"{"foo": 123}"#).unwrap(),
            from_snbt_str(r#"{"foo": 123L}"#)
        )
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(
            parse_compound(r#"{"foo": 123.45}"#).unwrap(),
            from_snbt_str(r#"{"foo": 123.45}"#)
        )
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            parse_compound(
                r#" { "foo" : "bar" , "list" : [ "a" , "b" ] , "long" : 1 , "double" : 1.2 } "#
            )
            .unwrap(),
            from_snbt_str(r#"{"foo": "bar", "list": ["a", "b"], "long": 1L, "double": 1.2}"#)
        )
    }

    #[test]
    fn test_string_escapes() {
        assert_eq!(
            parse_compound(r#"{"foo": "\\\n\r\t\"\u0020"}"#).unwrap(),
            from_snbt_str("{\"foo\": \"\\\\\n\r\t\\\" \"}")
        )
    }
}
