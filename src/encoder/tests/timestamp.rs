#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::str::FromStr;

    use chrono::{NaiveDateTime, DateTime, Local};

    assert_write!(naive_date_time, write_timestamp,
        NaiveDateTime::from_str("2019-01-27T13:28:00").unwrap(),
        vec![0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6f, 0x4c, 0x30, 0x58, 0x00]
    );

    assert_write!(tz_date_time, write_timestamp_with_time_zone,
        DateTime::<Local>::from_str("2019-01-27T13:28:00+03:00").unwrap(),
        vec![0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x23, 0x6c, 0xc8, 0x75, 0x6c, 0x00]
    );
}
