use std::time::Duration;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{space0, u64},
    combinator::{eof, opt},
    error::ParseError,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

fn fragment<'a, E, S>(suffixes: S) -> impl FnMut(&'a str) -> IResult<&'a str, u64, E>
where
    E: ParseError<&'a str>,
    S: Parser<&'a str, &'a str, E>,
{
    terminated(u64, preceded(space0, suffixes))
}

fn hour_suffixes(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("hours"),
        tag_no_case("hour"),
        tag_no_case("hrs"),
        tag_no_case("hr"),
        tag_no_case("h"),
    ))(input)
}

fn minute_suffixes(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("minutes"),
        tag_no_case("minute"),
        tag_no_case("mins"),
        tag_no_case("min"),
        tag_no_case("mi"),
        tag_no_case("m"),
    ))(input)
}

fn second_suffixes(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("seconds"),
        tag_no_case("second"),
        tag_no_case("secs"),
        tag_no_case("sec"),
        tag_no_case("se"),
        tag_no_case("s"),
    ))(input)
}

fn separator(input: &str) -> IResult<&str, &str> {
    delimited(
        space0,
        alt((
            preceded(opt(pair(tag(","), space0)), tag_no_case("and")),
            tag(":"),
            tag(","),
            tag(";"),
            space0,
        )),
        space0,
    )(input)
}

fn parse_time_full(input: &str) -> IResult<&str, Duration> {
    let (input, (hours, _, minutes, _, seconds)) = terminated(
        tuple((
            opt(fragment(hour_suffixes)),
            separator,
            opt(fragment(minute_suffixes)),
            separator,
            opt(fragment(second_suffixes)),
        )),
        eof,
    )(input)?;
    let total_seconds =
        hours.unwrap_or(0) * 3600 + minutes.unwrap_or(0) * 60 + seconds.unwrap_or(0);

    Ok((input, Duration::from_secs(total_seconds)))
}

pub fn parse(input: &str) -> Result<Duration, nom::Err<nom::error::Error<&str>>> {
    parse_time_full(input).map(|(_, duration)| duration)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        assert_eq!(super::parse("1h"), Ok(std::time::Duration::from_secs(3600)));
        assert_eq!(
            super::parse("1h 30m"),
            Ok(std::time::Duration::from_secs(5400))
        );
        assert_eq!(
            super::parse("1h30m15s"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1h, 30m and 15s"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1h, 30m, and 15s"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1h:30m:15s"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1h;30m;15s"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1hour 30minutes and 15seconds"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1 hour 30 minutes and 15 seconds"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1 hour 30 minutes, and 15 seconds"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1 hour 30 minutes 15 seconds"),
            Ok(std::time::Duration::from_secs(5415))
        );
        assert_eq!(
            super::parse("1 hour 15 seconds"),
            Ok(std::time::Duration::from_secs(3615))
        );
    }
}
