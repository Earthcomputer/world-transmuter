use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_a, tag};
use nom::character::complete::{char, satisfy, space1};
use nom::combinator::{map, map_opt, map_res, recognize, value};
use nom::error::Error;
use nom::multi::{many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::{AsChar, Finish, IResult};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use valence_nbt::value::ValueRef;
use valence_nbt::{Compound, List, Value};

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
pub fn stringify_compound(map: Compound, round_trip: bool, pretty: bool) -> String {
    let mut str = String::new();
    stringify(
        ValueRef::Compound(&map),
        &mut str,
        round_trip,
        pretty,
        Indent(0),
    )
    .expect("Should not get Err writing to String");
    str
}

fn stringify(
    obj: ValueRef,
    str: &mut String,
    round_trip: bool,
    pretty: bool,
    mut indent: Indent,
) -> std::fmt::Result {
    match obj {
        ValueRef::Byte(b) => write!(str, "{}", b)?,
        ValueRef::Short(s) => write!(str, "{}", s)?,
        ValueRef::Int(i) => write!(str, "{}", i)?,
        ValueRef::Long(l) => write!(str, "{}", l)?,
        ValueRef::Float(f) => write!(str, "{}", f)?,
        ValueRef::Double(d) => write!(str, "{}", d)?,
        ValueRef::ByteArray(arr) => {
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
        ValueRef::IntArray(arr) => {
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
        ValueRef::LongArray(arr) => {
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
        ValueRef::List(list) => {
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
        ValueRef::Compound(map) => {
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
        ValueRef::String(input) => stringify_string(input, str),
    }
    Ok(())
}

fn stringify_string(input: &str, output: &mut String) {
    output.push('"');
    for ch in input.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            _ => output.push(ch),
        }
    }
    output.push('"');
}

/// If `round_trip` is true, encodes `false`, `true` and `null` as `[0]`, `[1]` and `[2]` byte arrays respectively.
pub fn parse_compound(json: &str, round_trip: bool) -> Result<Compound, ParseError> {
    preceded(space, |i| object(i, round_trip))(json)
        .finish()
        .map(|(_, o)| o)
        .map_err(|err| {
            ParseError(Error {
                input: err.input.to_owned(),
                code: err.code,
            })
        })
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), many0(alt((space1, is_a("\r\n")))))(i)
}

fn any(i: &str, round_trip: bool) -> IResult<&str, Value> {
    alt((
        map(|i| object(i, round_trip), Value::Compound),
        map(|i| array(i, round_trip), Value::List),
        map(string, Value::String),
        map_res(terminated(recognize_float, space), |str| {
            Result::<_, <f64 as FromStr>::Err>::Ok(match str::parse::<i64>(str) {
                Ok(long) => Value::Long(long),
                Err(_) => Value::Double(str::parse::<f64>(str)?),
            })
        }),
        map(pair(tag("false"), space), |_| {
            if round_trip {
                Value::ByteArray(vec![0])
            } else {
                Value::Byte(0)
            }
        }),
        map(pair(tag("true"), space), |_| {
            if round_trip {
                Value::ByteArray(vec![1])
            } else {
                Value::Byte(1)
            }
        }),
        map(pair(tag("null"), space), |_| {
            if round_trip {
                Value::ByteArray(vec![2])
            } else {
                Value::Byte(0)
            }
        }),
    ))(i)
}

fn object(i: &str, round_trip: bool) -> IResult<&str, Compound> {
    map(
        delimited(
            pair(char('{'), space),
            separated_list0(
                pair(char(','), space),
                separated_pair(string, pair(char(':'), space), |i| any(i, round_trip)),
            ),
            pair(char('}'), space),
        ),
        |vec| {
            let mut map = Compound::new();
            for (k, v) in vec {
                map.insert(k, v);
            }
            map
        },
    )(i)
}

fn array(i: &str, round_trip: bool) -> IResult<&str, List> {
    map_res(
        delimited(
            pair(char('['), space),
            separated_list0(pair(char(','), space), |i| any(i, round_trip)),
            pair(char(']'), space),
        ),
        |vec| {
            let mut list = List::new();
            for v in vec {
                if !list.try_push(v) {
                    return Err(());
                }
            }
            Ok(list)
        },
    )(i)
}

fn string(i: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            satisfy(|ch| ch != '\\' && ch != '"' && ch != '\n' && ch != '\r'),
            '\\',
            alt((
                value('\\', char('\\')),
                value('"', char('"')),
                value('\'', char('\'')),
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                map_opt(
                    preceded(
                        char('u'),
                        recognize(tuple((hex_digit, hex_digit, hex_digit, hex_digit))),
                    ),
                    |str| char::from_u32(u32::from_str_radix(str, 16).unwrap()),
                ),
            )),
        ),
        pair(char('"'), space),
    )(i)
}

fn hex_digit(i: &str) -> IResult<&str, ()> {
    value((), satisfy(|ch| ch.is_hex_digit()))(i)
}

#[cfg(test)]
mod tests {
    use super::parse_compound;
    use valence_nbt::snbt::from_snbt_str;
    use valence_nbt::Value;

    #[test]
    fn test_parse_object() {
        assert_eq!(
            Value::Compound(parse_compound(r#"{"foo": "bar", "baz": "quux"}"#, false).unwrap()),
            from_snbt_str(r#"{"foo": "bar", "baz": "quux"}"#).unwrap()
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            Value::Compound(parse_compound(r#"{"foo": ["bar", "baz", "quux"]}"#, false).unwrap()),
            from_snbt_str(r#"{"foo": ["bar", "baz", "quux"]}"#).unwrap()
        )
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(
            Value::Compound(parse_compound(r#"{"foo": 123}"#, false).unwrap()),
            from_snbt_str(r#"{"foo": 123L}"#).unwrap()
        )
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(
            Value::Compound(parse_compound(r#"{"foo": 123.45}"#, false).unwrap()),
            from_snbt_str(r#"{"foo": 123.45}"#).unwrap()
        )
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            Value::Compound(
                parse_compound(
                    r#" { "foo" : "bar" , "list" : [ "a" , "b" ] , "long" : 1 , "double" : 1.2 } "#,
                    false,
                )
                .unwrap()
            ),
            from_snbt_str(r#"{"foo": "bar", "list": ["a", "b"], "long": 1L, "double": 1.2}"#)
                .unwrap()
        )
    }

    #[test]
    fn test_string_escapes() {
        assert_eq!(
            Value::Compound(parse_compound(r#"{"foo": "\\\n\r\t\"\u0020"}"#, false).unwrap()),
            from_snbt_str(r#"{"foo": "\\\n\r\t\" "}"#).unwrap()
        )
    }
}
