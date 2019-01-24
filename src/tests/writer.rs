use std::str::FromStr;

use uuid::Uuid;

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
    assert!(writer.write_header().is_ok());

    assert!(writer.write_tuple(3).is_ok());
    assert!(writer.write_bool(false).is_ok());
    assert!(writer.write_bool(true).is_ok());
    assert!(writer.write_bool(false).is_ok());

    assert!(writer.write_trailer().is_ok());

    let expected = vec![
        0x50u8, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00, // PGCOPY\n\xff\r\n\0
        0x00, 0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, //extension area length

        // One tuple
        0x00, 0x03, // fields count
        0x00, 0x00, 0x00, 0x01, 0x00, // false
        0x00, 0x00, 0x00, 0x01, 0x01, // true
        0x00, 0x00, 0x00, 0x01, 0x00, // false

        0xff, 0xff // trailer
    ];

    assert_eq!(&expected, writer.inner());
}

#[test]
fn test_uuid() {
    let buf: Vec<u8> = vec![];
    let mut writer = Writer::new(buf);
    assert!(writer.write_header().is_ok());

    assert!(writer.write_tuple(2).is_ok());
    assert!(writer.write_uuid(Uuid::from_str("1d662762-2010-11e9-ad8b-c869cdb5cd46").unwrap()).is_ok());
    assert!(writer.write_uuid(Uuid::from_str("e11b8974-5245-4eca-a06b-a9a440074131").unwrap()).is_ok());

    assert!(writer.write_trailer().is_ok());

    let expected = vec![
        0x50u8, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00, // PGCOPY\n\xff\r\n\0
        0x00, 0x00, 0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, //extension area length

        // One tuple
        0x00, 0x02, // fields count
        0x00, 0x00, 0x00, 0x10, 0x1d, 0x66, 0x27, 0x62, 0x20, 0x10, 0x11, 0xe9, 0xad, 0x8b, 0xc8, 0x69, 0xcd, 0xb5, 0xcd, 0x46,
        0x00, 0x00, 0x00, 0x10, 0xe1, 0x1b, 0x89, 0x74, 0x52, 0x45, 0x4e, 0xca, 0xa0, 0x6b, 0xa9, 0xa4, 0x40, 0x07, 0x41, 0x31,

        0xff, 0xff // trailer
    ];

    assert_eq!(&expected, writer.inner());
}


