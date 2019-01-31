#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::str::FromStr;

    use chrono::NaiveTime;

    assert_write!(naive_time, write_time,
        NaiveTime::from_str("13:28:01.789").unwrap(),
        vec![0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0b, 0x49, 0xbd, 0x64, 0x48]
    );
}
