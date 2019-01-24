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
