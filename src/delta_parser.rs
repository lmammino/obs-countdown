use std::time::Duration;

use winnow::{
    ascii::{digit1, space0, Caseless},
    combinator::{alt, delimited, opt, terminated},
    error::{ContextError, ParseError},
    PResult, Parser,
};

fn fragment<'s>(
    suffixes: impl Parser<&'s str, (), ContextError>,
) -> impl Parser<&'s str, u64, ContextError> {
    terminated(digit1, (space0, suffixes)).parse_to()
}

fn hour_suffixes(input: &mut &str) -> PResult<()> {
    alt((
        Caseless("hours"),
        Caseless("hour"),
        Caseless("hrs"),
        Caseless("hr"),
        Caseless("h"),
    ))
    .void()
    .parse_next(input)
}

fn minute_suffixes(input: &mut &str) -> PResult<()> {
    alt((
        Caseless("minutes"),
        Caseless("minute"),
        Caseless("mins"),
        Caseless("min"),
        Caseless("mi"),
        Caseless("m"),
    ))
    .void()
    .parse_next(input)
}

fn second_suffixes(input: &mut &str) -> PResult<()> {
    alt((
        Caseless("seconds"),
        Caseless("second"),
        Caseless("secs"),
        Caseless("sec"),
        Caseless("se"),
        Caseless("s"),
    ))
    .void()
    .parse_next(input)
}

fn separator(input: &mut &str) -> PResult<()> {
    delimited(
        space0,
        alt((
            (opt((',', space0)), Caseless("and")).void(),
            ':'.void(),
            ','.void(),
            ';'.void(),
            space0.void(),
        )),
        space0,
    )
    .parse_next(input)
}

fn parse_time_full(input: &mut &str) -> PResult<Duration> {
    let (hours, minutes, seconds) = (
        opt(terminated(fragment(hour_suffixes), separator)),
        opt(terminated(fragment(minute_suffixes), separator)),
        opt(fragment(second_suffixes)),
    )
        .parse_next(input)?;
    let total_seconds =
        hours.unwrap_or(0) * 3600 + minutes.unwrap_or(0) * 60 + seconds.unwrap_or(0);

    Ok(Duration::from_secs(total_seconds))
}

pub fn parse(input: &str) -> Result<Duration, ParseError<&str, ContextError>> {
    parse_time_full.parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;
        use std::time::Duration;

        assert_eq!(parse("1h"), Ok(Duration::from_secs(3600)));
        assert_eq!(parse("1h 30m"), Ok(Duration::from_secs(5400)));
        assert_eq!(parse("1h30m15s"), Ok(Duration::from_secs(5415)));
        assert_eq!(parse("1h, 30m and 15s"), Ok(Duration::from_secs(5415)));
        assert_eq!(parse("1h, 30m, and 15s"), Ok(Duration::from_secs(5415)));
        assert_eq!(parse("1h:30m:15s"), Ok(Duration::from_secs(5415)));
        assert_eq!(parse("1h;30m;15s"), Ok(Duration::from_secs(5415)));
        assert_eq!(
            parse("1hour 30minutes and 15seconds"),
            Ok(Duration::from_secs(5415))
        );
        assert_eq!(
            parse("1 hour 30 minutes and 15 seconds"),
            Ok(Duration::from_secs(5415))
        );
        assert_eq!(
            parse("1 hour 30 minutes, and 15 seconds"),
            Ok(Duration::from_secs(5415))
        );
        assert_eq!(
            parse("1 hour 30 minutes 15 seconds"),
            Ok(Duration::from_secs(5415))
        );
        assert_eq!(parse("1 hour 15 seconds"), Ok(Duration::from_secs(3615)));
    }
}
