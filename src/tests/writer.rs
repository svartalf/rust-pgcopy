use std::str::FromStr;

use uuid::Uuid;
use chrono::{NaiveTime, NaiveDate, NaiveDateTime, DateTime, Local};

use crate::Writer;

#[test]
fn test_empty_copy() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);
    assert!(writer.write_header().is_ok());
    assert!(writer.write_trailer().is_ok());

    let expected = vec![
        0x50u8, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00, // PGCOPY\n\xff\r\n\0
        0x00, 0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, //extension area length
        0xff, 0xff // trailer
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_bool() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);
    assert!(writer.write_bool(false).is_ok());
    assert!(writer.write_bool(true).is_ok());
    assert!(writer.write_bool(false).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x01, 0x00, // false
        0x00, 0x00, 0x00, 0x01, 0x01, // true
        0x00, 0x00, 0x00, 0x01, 0x00, // false
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_uuid() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);

    assert!(writer.write_uuid(Uuid::from_str("1d662762-2010-11e9-ad8b-c869cdb5cd46").unwrap()).is_ok());
    assert!(writer.write_uuid(Uuid::from_str("e11b8974-5245-4eca-a06b-a9a440074131").unwrap()).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x10, 0x1d, 0x66, 0x27, 0x62, 0x20, 0x10, 0x11, 0xe9, 0xad, 0x8b, 0xc8, 0x69, 0xcd, 0xb5, 0xcd, 0x46,
        0x00, 0x00, 0x00, 0x10, 0xe1, 0x1b, 0x89, 0x74, 0x52, 0x45, 0x4e, 0xca, 0xa0, 0x6b, 0xa9, 0xa4, 0x40, 0x07, 0x41, 0x31,
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_timestamp() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);

    let datetime = NaiveDateTime::from_str("2019-01-27T13:28:00").unwrap();
    assert!(writer.write_timestamp(datetime).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6f, 0x4c, 0x30, 0x58, 0x00,
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_timestamp_with_time_zone() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);

    let datetime = DateTime::<Local>::from_str("2019-01-27T13:28:00+03:00").unwrap();
    assert!(writer.write_timestamp_with_time_zone(datetime).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6c, 0xc8, 0x75, 0x6c, 0x00,
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_date() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);

    let date = NaiveDate::from_str("2019-01-27").unwrap();
    assert!(writer.write_date(date).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1b, 0x36,
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_time() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);

    let time = NaiveTime::from_str("13:28:01.789").unwrap();
    assert!(writer.write_time(time).is_ok());

    let expected = vec![
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0b, 0x49, 0xbd, 0x64, 0x48
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_complex() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);
    assert!(writer.write_header().is_ok());

    let datetime = NaiveDate::from_ymd(1990, 07, 18).and_hms(2, 3, 10);

    assert!(writer.write_tuple(8).is_ok());

    assert!(writer.write_bool(true).is_ok());
    assert!(writer.write_i32(1).is_ok());
    assert!(writer.write_i64(100).is_ok());
    assert!(writer.write_f32(3.14).is_ok());
    assert!(writer.write_f64(3.14).is_ok());
    assert!(writer.write_str("one").is_ok());
    assert!(writer.write_timestamp(datetime).is_ok());
    assert!(writer.write_bytes(b"\x61\x62\x63\x20\x6b\x6c\x6d\x20\x2a\xa9\x54").is_ok());


    assert!(writer.write_trailer().is_ok());

    let expected = vec![
        0x50u8, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00, // PGCOPY\n\xff\r\n\0
        0x00, 0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, //extension area length

        // One tuple
        0x00, 0x08, // fields count
        0x00, 0x00, 0x00, 0x01, 0x01,
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
        0x00, 0x00, 0x00, 0x04, 0x40, 0x48, 0xf5, 0xc3,
        0x00, 0x00, 0x00, 0x08, 0x40, 0x09, 0x1e, 0xb8, 0x51, 0xeb, 0x85, 0x1f,
        0x00, 0x00, 0x00, 0x03, 0x6f, 0x6e, 0x65,
        0x00, 0x00, 0x00, 0x08, 0xff, 0xfe, 0xf0, 0x97, 0x18, 0x99, 0x33, 0x80,
        0x00, 0x00, 0x00, 0x0b, 0x61, 0x62, 0x63, 0x20, 0x6b, 0x6c, 0x6d, 0x20, 0x2a, 0xa9, 0x54,

        0xff, 0xff // trailer
    ];

    assert_eq!(&expected, writer.inner());
}
