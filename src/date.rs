use std::str::FromStr;

use chrono::{Datelike, Local};
use nom::Parser;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    pub fn today() -> Self {
        let time = Local::now();
        Self {
            year: time.year() as u32,
            month: time.month(),
            day: time.day(),
        }
    }
}

impl From<Date> for String {
    fn from(d: Date) -> Self {
        format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
    }
}

impl TryFrom<&str> for Date {
    type Error = ();

    fn try_from<'a>(s: &'a str) -> Result<Self, Self::Error> {
        use nom::{
            character::complete::{char, digit1},
            combinator::{all_consuming, map_res},
            sequence::tuple,
        };

        // TODO restrict it to yyyy-mm-dd? Right now 1-1-1 would be valid
        let u32_parser = || {
            map_res(digit1::<_, ()>, |s: &str| {
                Ok::<u32, ()>(u32::from_str(s).expect("Could not parse string to usize"))
            })
        };
        let (_, (year, _, month, _, day)) = all_consuming(tuple((
            u32_parser(),
            char('-'),
            u32_parser(),
            char('-'),
            u32_parser(),
        )))
        .parse(s)
        .map_err(|_| ())?;

        Ok(Date { year, month, day })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_dates() {
        assert_eq!(
            Date::try_from("2022-01-30"),
            Ok(Date {
                year: 2022,
                month: 1,
                day: 30,
            })
        );
        assert_eq!(
            Date::try_from("2022-03-01"),
            Ok(Date {
                year: 2022,
                month: 3,
                day: 1,
            })
        );
        assert_eq!(
            Date::try_from("2022-12-31"),
            Ok(Date {
                year: 2022,
                month: 12,
                day: 31,
            })
        );
        assert_eq!(
            Date::try_from("2023-01-20"),
            Ok(Date {
                year: 2023,
                month: 1,
                day: 20,
            })
        );
    }

    #[test]
    #[should_panic]
    fn parse_template() {
        let _ = Date::try_from("template").unwrap();
    }
}
