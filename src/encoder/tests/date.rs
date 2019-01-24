#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::str::FromStr;

    use chrono::NaiveDate;

    assert_write!(naive_date, write_date,
        NaiveDate::from_str("2019-01-27").unwrap(),
        vec![0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x1b, 0x36]
    );
}
