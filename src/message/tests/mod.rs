use std::str::FromStr;

use reqwest::Url;
use time::{Date, Month};

use crate::message::SwitrsData;

const EMAIL: &[u8] = include_bytes!("i-switrs-email.raw");

#[test]
fn test_parse_switrs_msg() {
    let data = SwitrsData::parse(EMAIL).expect("failed to parse email");

    assert_eq!(data.request_id.expect("no request id"), 240014);
    assert_eq!(data.report_type.expect("no report type"), "Raw Data");
    assert_eq!(data.jurisdiction.expect("no jurisdiction"), "ALL");
    assert_eq!(
        data.reporting_period.expect("no reporting period"),
        Date::from_calendar_date(2010, Month::January, 1).unwrap()
            ..Date::from_calendar_date(2023, Month::December, 1).unwrap()
    );
    assert!(data.contains_long_lat);
    assert!(data.contains_header);
    assert_eq!(
        data.db_url.expect("no reporting period"),
        Url::from_str("https://iswitrsreports.chp.ca.gov/reports/4481761401380215189.zip").unwrap()
    )
}
