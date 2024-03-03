use std::time::Duration;

pub fn format(duration: Duration, format: &str) -> String {
    let mut remaining = duration.as_secs();
    let hours = remaining / 3600;
    remaining %= 3600;
    let minutes = remaining / 60;
    remaining %= 60;
    let seconds = remaining;

    let mut formatted = format.replace("%h", &hours.to_string());
    formatted = formatted.replace("%H", &format!("{:02}", hours));
    formatted = formatted.replace("%m", &minutes.to_string());
    formatted = formatted.replace("%M", &format!("{:02}", minutes));
    formatted = formatted.replace("%s", &seconds.to_string());
    formatted = formatted.replace("%S", &format!("{:02}", seconds));

    formatted
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        // zero-padded
        assert_eq!(format(Duration::from_secs(0), "%H:%M:%S"), "00:00:00");
        assert_eq!(format(Duration::from_secs(1), "%H:%M:%S"), "00:00:01");
        assert_eq!(format(Duration::from_secs(60), "%H:%M:%S"), "00:01:00");
        assert_eq!(format(Duration::from_secs(61), "%H:%M:%S"), "00:01:01");
        assert_eq!(format(Duration::from_secs(3600), "%H:%M:%S"), "01:00:00");
        assert_eq!(format(Duration::from_secs(3601), "%H:%M:%S"), "01:00:01");
        assert_eq!(format(Duration::from_secs(3661), "%H:%M:%S"), "01:01:01");
        assert_eq!(
            format(
                Duration::from_secs(3661),
                "%H hour(s) %M minute(s) %S second(s)"
            ),
            "01 hour(s) 01 minute(s) 01 second(s)"
        );
        assert_eq!(
            format(Duration::from_secs(1000000), "%H:%M:%S"),
            "277:46:40"
        );
        // not zero-padded
        assert_eq!(format(Duration::from_secs(0), "%h:%m:%s"), "0:0:0");
        assert_eq!(format(Duration::from_secs(1), "%h:%m:%s"), "0:0:1");
        assert_eq!(format(Duration::from_secs(60), "%h:%m:%s"), "0:1:0");
        assert_eq!(format(Duration::from_secs(61), "%h:%m:%s"), "0:1:1");
        assert_eq!(format(Duration::from_secs(3600), "%h:%m:%s"), "1:0:0");
        assert_eq!(format(Duration::from_secs(3601), "%h:%m:%s"), "1:0:1");
        assert_eq!(format(Duration::from_secs(3661), "%h:%m:%s"), "1:1:1");
        assert_eq!(
            format(
                Duration::from_secs(3661),
                "%h hour(s) %m minute(s) %s second(s)"
            ),
            "1 hour(s) 1 minute(s) 1 second(s)"
        );
        assert_eq!(
            format(Duration::from_secs(1000000), "%h:%m:%s"),
            "277:46:40"
        );
    }
}
