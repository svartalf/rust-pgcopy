macro_rules! assert_write {
    ($test_name:ident, $method:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            use crate::Encoder;

            let buf: Vec<u8> = vec![];
            let mut writer = Encoder::new(buf);

            let result = writer.$method($value);

            assert!(result.is_ok());
            assert_eq!(&$expected, writer.get_ref());
        }
    }
}

macro_rules! assert_i16 {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_smallint, $value, $expected);
    };
}
macro_rules! assert_i32 {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_int, $value, $expected);
    };
}
macro_rules! assert_i64 {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_bigint, $value, $expected);
    };
}
macro_rules! assert_f32 {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_real, $value, $expected);
    };
}
macro_rules! assert_f64 {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_double, $value, $expected);
    };
}
macro_rules! assert_bool {
    ($test_name:ident, $value:expr, $expected:expr) => {
        assert_write!($test_name, write_bool, $value, $expected);
    };
}
//macro_rules! assert_numeric {
//    ($test_name:ident, $value:expr, $expected:expr) => {
//        assert_write!($test_name, write_numeric, $value, $expected);
//    };
//}

// TODO:
// assert_write!(null, write_null(), vec![0xff, 0xff, 0xff, 0xff]);

mod numeric;
mod boolean;
mod bytes;
mod text;
mod timestamp;
mod date;
mod time;
mod uuid;
mod macaddr;
mod macaddr8;

