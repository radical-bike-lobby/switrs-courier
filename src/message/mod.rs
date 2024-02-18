#[cfg(test)]
mod tests;

use std::{ops::Range, str::FromStr};

use mailparse::parse_mail;
use regex::Regex;
use reqwest::Url;
use time::{Date, Month};

use crate::Error;

#[derive(Debug)]
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

        // id is in the subject
        eprintln!("subject: {subject:?}");
        let request_id = subject.as_ref().and_then(|subject| parse_id(subject));
        eprintln!("request_id: {request_id:?}");

        // the rest is in the body, first have to find the right body
        let data;
        let mut search = vec![msg];
        loop {
            let msg = search.pop().ok_or(Error::NoDataFound())?;

            // we'll stop searching if we find the first db_url in the message
            let body = msg.get_body()?;
            let tmp = parse_body(&body, request_id);

            if tmp.db_url.is_some() {
                data = tmp;
                break;
            }

            search.extend(msg.subparts);
        }

        eprintln!("parsed: {data:?}");
        Ok(data)
    }
}

/// Parses the request id from the Subject, expected: `I-SWITRS Report Request - ID #240014`
fn parse_id(subject: &str) -> Option<usize> {
    let re = Regex::new(r"ID #([0-9]+)").unwrap();

    let captures = re.captures(subject)?;
    let id_str = captures.get(1).map(|m| m.as_str().trim())?;

    usize::from_str(id_str)
        .inspect_err(|error| eprintln!("failed to parse id, {id_str}: {error}"))
        .ok()
}

fn parse_body(body: &str, request_id: Option<usize>) -> SwitrsData {
    // Report Type:               Raw Data
    let re_report_type = Regex::new(r"Report\sType\:\s+(.+)").unwrap();
    // Jurisdiction:              ALL
    let re_jurisdiction = Regex::new(r"Jurisdiction:\s+(.+)").unwrap();
    // Reporting Period:          01/01/2010 - 12/01/2023
    let re_reporting_period =
        Regex::new(r"Reporting Period:\s+(\d\d/\d\d/\d\d\d\d)\s+-\s+(\d\d/\d\d/\d\d\d\d)").unwrap();
    // Lat/Long:                  Yes
    let re_lat_long = Regex::new(r"Lat/Long:\s+(\w+)").unwrap();
    // Header:                    Yes
    let re_header = Regex::new(r"Header:\s+(\w+)").unwrap();
    // https://iswitrsreports.chp.ca.gov/reports/4481761401380215189.zip
    let re_db_url = Regex::new(r"https://.+/reports/.+").unwrap();

    let mut report_type = None::<String>;
    let mut jurisdiction = None::<String>;
    let mut reporting_period = None::<Range<Date>>;
    let mut contains_long_lat = None::<bool>;
    let mut contains_header = None::<bool>;
    let mut db_url = None::<Url>;

    for line in body.lines() {
        // report type
        if let Some(s) = re_report_type
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
        {
            report_type = Some(s.to_string());
        }

        // jurisdiction
        if let Some(s) = re_jurisdiction
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
        {
            jurisdiction = Some(s.to_string());
        }

        // reporting_period
        if let Some((s1, s2)) = re_reporting_period
            .captures(line)
            .map(|caps| (caps.get(1), caps.get(2)))
            .map(|(m1, m2)| {
                (
                    m1.map_or("", |s| s.as_str().trim()),
                    m2.map_or("", |s| s.as_str().trim()),
                )
            })
        {
            let start = parse_date(s1);
            let end = parse_date(s2);

            if let (Some(start), Some(end)) = (start, end) {
                reporting_period = Some(Range { start, end })
            }
        }

        // contains_long_lat
        if let Some(s) = re_lat_long
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
        {
            contains_long_lat = Some(parse_yes_no(s));
        }

        // contains_header
        if let Some(s) = re_header
            .captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
        {
            contains_header = Some(parse_yes_no(s));
        }

        // db_url (no captures in this case)
        if let Some(s) = re_db_url
            .captures(line)
            .and_then(|caps| caps.get(0))
            .map(|m| m.as_str().trim())
        {
            db_url = Url::from_str(s).ok();
        }
    }

    SwitrsData {
        request_id,
        report_type,
        jurisdiction,
        reporting_period,
        contains_long_lat: contains_long_lat.unwrap_or_default(),
        contains_header: contains_header.unwrap_or_default(),
        db_url,
    }
}

fn parse_yes_no(input: &str) -> bool {
    input == "Yes" || bool::from_str(input).unwrap_or_default()
}

fn parse_date(input: &str) -> Option<Date> {
    let date_strs = input.splitn(3, '/').collect::<Vec<&str>>();
    let month: u8 = date_strs
        .first()?
        .parse()
        .inspect_err(|e| eprintln!("bad month: {input}"))
        .ok()?;
    let day: u8 = date_strs
        .get(1)?
        .parse()
        .inspect_err(|e| eprintln!("bad day: {input}"))
        .ok()?;
    let year: i32 = date_strs
        .get(2)?
        .parse()
        .inspect_err(|e| eprintln!("bad year: {input}"))
        .ok()?;

    let month = Month::January.nth_next(month - 1);

    Date::from_calendar_date(year, month, day)
        .inspect_err(|e| eprintln!("bad date, {input}: {e}"))
        .ok()
}
