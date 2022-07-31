use std::str::FromStr;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

use super::today;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub(crate) enum DateSpecifier {
    Today,
    Tomorrow,
    This(Weekday),
    Next(Weekday),
    Date(chrono::NaiveDate),
}

fn parse_weekday(s: &str) -> anyhow::Result<Weekday> {
    match s {
        "monday" => Ok(Weekday::Mon),
        "tuesday" => Ok(Weekday::Tue),
        "wednesday" => Ok(Weekday::Wed),
        "thursday" => Ok(Weekday::Thu),
        "friday" => Ok(Weekday::Fri),
        "saturday" => Ok(Weekday::Sat),
        "sunday" => Ok(Weekday::Sun),
        s => Err(anyhow::anyhow!("{s} is not a valid day of the week")),
    }
}

impl FromStr for DateSpecifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "today" {
            Ok(Self::Today)
        } else if s == "tomorrow" {
            Ok(Self::Tomorrow)
        } else if let Some(stripped) = s.strip_prefix("this/") {
            parse_weekday(stripped).map(Self::This)
        } else if let Some(stripped) = s.strip_prefix("next/") {
            parse_weekday(stripped).map(Self::Next)
        } else {
            chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y")
                .map_err(|_| anyhow::anyhow!("fixed date must be given as e.g. 31/10/2022"))
                .map(Self::Date)
        }
    }
}

impl From<DateSpecifier> for NaiveDate {
    fn from(spec: DateSpecifier) -> Self {
        let today = today();

        match spec {
            DateSpecifier::This(day) => {
                let delta = day.num_days_from_monday() as i64 - today.weekday().num_days_from_monday() as i64;
                today + Duration::days(delta)
            }
            DateSpecifier::Next(day) => {
                let delta = day.num_days_from_monday() as i64 - today.weekday().num_days_from_monday() as i64;
                today + Duration::days(delta + 7)
            }
            DateSpecifier::Date(date) => date,
            DateSpecifier::Today => today,
            DateSpecifier::Tomorrow => today.succ(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_specifier_from_str() {
        assert_eq!(
            DateSpecifier::Today,
            DateSpecifier::from_str("today").expect("could not parse")
        );
        assert_eq!(
            DateSpecifier::Tomorrow,
            DateSpecifier::from_str("tomorrow").expect("could not parse")
        );
        assert_eq!(
            DateSpecifier::This(Weekday::Sat),
            DateSpecifier::from_str("this/saturday").expect("could not parse")
        );
        assert_eq!(
            DateSpecifier::Next(Weekday::Sat),
            DateSpecifier::from_str("next/saturday").expect("could not parse")
        );
        assert_eq!(
            DateSpecifier::Date(today()),
            DateSpecifier::from_str("05/01/2022").expect("could not parse")
        );
    }

    #[test]
    fn date_specifier_to_date() {
        assert_eq!(
            NaiveDate::from(DateSpecifier::Today),
            NaiveDate::from_isoywd(2022, 1, Weekday::Wed)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::Tomorrow),
            NaiveDate::from_isoywd(2022, 1, Weekday::Thu)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::This(Weekday::Mon)),
            NaiveDate::from_isoywd(2022, 1, Weekday::Mon)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::This(Weekday::Sun)),
            NaiveDate::from_isoywd(2022, 1, Weekday::Sun)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::Next(Weekday::Mon)),
            NaiveDate::from_isoywd(2022, 2, Weekday::Mon)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::Next(Weekday::Sun)),
            NaiveDate::from_isoywd(2022, 2, Weekday::Sun)
        );
        assert_eq!(
            NaiveDate::from(DateSpecifier::Date(today())),
            NaiveDate::from_isoywd(2022, 1, Weekday::Wed)
        );
    }
}
