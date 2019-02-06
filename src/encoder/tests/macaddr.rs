assert_write!(macaddr, write_macaddr,
    [0xde, 0xaa, 0xad, 0xbe, 0xee, 0xef],
    vec![0x00, 0x00, 0x00, 0x06, 0xde, 0xaa, 0xad, 0xbe, 0xee, 0xef]);


#[cfg(feature = "with-eui48")]
mod with_eui48 {
    use eui48::MacAddress;

    assert_write!(macaddr, write_macaddr,
        MacAddress::new([0xde, 0xaa, 0xad, 0xbe, 0xee, 0xef]),
        vec![0x00, 0x00, 0x00, 0x06, 0xde, 0xaa, 0xad, 0xbe, 0xee, 0xef]
    );
}
