use std::cmp;

use num_bigint::{Sign, BigInt};
use ::bigdecimal::BigDecimal;

use crate::types::*;

impl PgNumeric for BigDecimal {
//    fn to_numeric(&self) -> (PgNumericWeight, PgNumericSign, PgNumericDscale, PgNumericDigits, i64) {
//        let (mut value, exponent) = self.as_bigint_and_exponent();
//        let mut digits: Vec<_> = value.to_str_radix(10)
//            .chars()
//            .filter_map(|x| x.to_digit(10))
//            .map(|x| x as i16)
//            .collect();
//
//        // TODO: Add check if `exponent` exceeds `i16` size
//        let dscale = -cmp::min(0, exponent) as i16;
//        let sign = match self.sign() {
//            Sign::Plus => PgNumericSign::Positive,
//            Sign::NoSign => PgNumericSign::Positive,
//            Sign::Minus => PgNumericSign::Negative,
//        };
//
//        (0, sign, dscale, digits, exponent)
//    }

    fn exponent(&self) -> i64 {
        let (_, exponent) = self.as_bigint_and_exponent();

        exponent * -1
    }

    fn sign(&self) -> PgNumericSign {
        match self.sign() {
            Sign::Plus => PgNumericSign::Positive,
            Sign::NoSign => PgNumericSign::Positive,
            Sign::Minus => PgNumericSign::Negative,
        }
    }

    fn digits(&self) -> Vec<u8> {
        let (value, _) = self.as_bigint_and_exponent();
        let digits: Vec<_> = value.to_str_radix(10)
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as u8)
            .collect();

        digits
    }
}
