assert_write!(static_str, write_str,
    "hello world",
    vec![0x00, 0x00, 0x00, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]);

assert_write!(heap_string, write_str,
    "hello world".to_string(),
    vec![0x00, 0x00, 0x00, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]);
