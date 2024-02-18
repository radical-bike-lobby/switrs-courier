#[cfg(test)]
mod tests;

use std::{ops::Range, str::FromStr};

use mailparse::parse_mail;
use regex::{Match, Regex};
use reqwest::Url;
use time::Date;

use crate::Error;

pub struct SwitrsData {
    pub request_id: Option<usize>,
    pub report_type: Option<String>,
    pub jurisdiction: Option<String>,
    pub reporting_period: Option<Range<Date>>,
    pub contains_long_lat: bool,
    pub contains_header: bool,
    pub db_url: Option<Url>,
}

impl SwitrsData {
    fn parse(data: &[u8]) -> Result<Self, Error> {
        let msg = parse_mail(data)?;

        let subject: Option<String> = msg
            .get_headers()
            .into_iter()
            .filter(|header| header.get_key_ref() == "Subject")
            .map(|header| header.get_value())
            .next();

        eprintln!("subject: {subject:?}");

        let id = subject.as_ref().and_then(|subject| parse_id(subject));

        eprintln!("request_id: {id:?}");

        unimplemented!()
    }
}

fn parse_id(subject: &str) -> Option<usize> {
    let re = Regex::new(r"ID #([0-9]+)").unwrap();

    let captures = re.captures(subject)?;
    let id_str = captures.get(1).map(|m| m.as_str().trim())?;

    usize::from_str(id_str)
        .inspect_err(|error| eprintln!("failed to parse id, {id_str}: {error}"))
        .ok()
}
