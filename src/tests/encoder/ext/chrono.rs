use std::str::FromStr;

use chrono::{NaiveTime, NaiveDate, NaiveDateTime, DateTime, Local};

use crate::Encoder;

#[test]
fn test_timestamp() {
    let buf: Vec<u8> = vec![];
    let mut writer = Encoder::new(buf);

    let datetime = NaiveDateTime::from_str("2019-01-27T13:28:00").unwrap();
    assert!(writer.write_timestamp(datetime).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6f, 0x4c, 0x30, 0x58, 0x00,
    ];

    assert_eq!(&expected, writer.get_ref());
}

#[test]
fn test_timestamp_with_time_zone() {
    let buf: Vec<u8> = vec![];
    let mut writer = Encoder::new(buf);

    let datetime = DateTime::<Local>::from_str("2019-01-27T13:28:00+03:00").unwrap();
    assert!(writer.write_timestamp_with_time_zone(datetime).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6c, 0xc8, 0x75, 0x6c, 0x00,
    ];

    assert_eq!(&expected, writer.get_ref());
}

#[test]
fn test_date() {
    let buf: Vec<u8> = vec![];
    let mut writer = Encoder::new(buf);

    let date = NaiveDate::from_str("2019-01-27").unwrap();
    assert!(writer.write_date(date).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1b, 0x36,
    ];

    assert_eq!(&expected, writer.get_ref());
}

#[test]
fn test_time() {
    let buf: Vec<u8> = vec![];
    let mut writer = Encoder::new(buf);

    let time = NaiveTime::from_str("13:28:01.789").unwrap();
    assert!(writer.write_time(time).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0b, 0x49, 0xbd, 0x64, 0x48
    ];

    assert_eq!(&expected, writer.get_ref());
}
