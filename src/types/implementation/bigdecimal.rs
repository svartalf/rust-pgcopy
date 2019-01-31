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
//
//
//        match value {
//            None => { // NaN case
//                self.inner.write_i32::<NetworkEndian>(8)?;
//                self.inner.write_i16::<NetworkEndian>(0)?; // Zero digits
//                self.inner.write_i16::<NetworkEndian>(0)?; // First digit weight
//                // PostgreSQL' C sources are using `0xc000` here, but it's overflowing for i16
//                self.inner.write_u16::<NetworkEndian>(0xc000)?; // NaN marker instead of sign
//                self.inner.write_u16::<NetworkEndian>(0) // Display scale is zero too
//            },
//            Some(val) => {
//                let exponent = val.exponent();
//                let sign = val.sign();
//                let mut raw_digits = val.digits();
//                let mut weight = 0;
//
//                for _ in 0..(exponent % 4 + 4) {
//                    raw_digits.push(0);
//                }
//                raw_digits.reverse();
//
//                let mut weight = 0;
//                let mut digits: Vec<i16> = vec![];
//                let mut offset = 0usize;
//
//                for (idx, chunk) in raw_digits.chunks(4).enumerate() {
//                    if chunk.iter().filter(|d| **d > 0).count() > 0 {
//                        break;
//                    } else {
//                        weight += 1;
//                        offset += 4;
//                    }
//                }
//
//                for (idx, chunk) in raw_digits[offset..].chunks(4).enumerate() {
//                    let mut res = 0;
//                    for (idx, digit) in chunk.iter().enumerate() {
//                        res += *digit as i16 * (10_i16.pow(idx as u32));
//                    }
//                    digits.insert(0, res);
//                }
//                println!("weight0: {}", weight);
//                println!("nte: {}", f32::floor(exponent as f32 / 4.0) as i16);
//                let ndigits = digits.len() as i16;
//                let dscale = -cmp::min(0, exponent);
//                weight += (f32::floor(exponent as f32 / 4.0) as i16) + ndigits - 1;
//
//                println!("ndigits: {:?}", ndigits);
//                println!("weight: {:?}", weight);
//                println!("sign: {:?}", sign);
//                println!("dscale: {:?}", dscale);
//                println!("exponent: {:?}", exponent);
//                println!("digits: {:?}", digits);
//
//                self.inner.write_i32::<NetworkEndian>(2 * (4 + digits.len() as i32))?;
//                self.inner.write_i16::<NetworkEndian>(digits.len() as i16)?;
//                self.inner.write_i16::<NetworkEndian>(weight)?;
//                let sign_repr = match sign {
//                    types::PgNumericSign::Positive => 0x0000,
//                    types::PgNumericSign::Negative => 0x4000,
//                };
//                self.inner.write_u16::<NetworkEndian>(sign_repr)?;
//                self.inner.write_u16::<NetworkEndian>(dscale as u16)?;
//                for digit in digits {
//                    self.inner.write_i16::<NetworkEndian>(digit);
//                }
//
//                Ok(())
//
//
////                let length = 2 + 2 + 2 + 2 + digits.len() as i32;
////                self.inner.write_i32::<NetworkEndian>(length)?;
////                self.inner.write_i16::<NetworkEndian>(digits.len() as i16)?;
////                self.inner.write_i16::<NetworkEndian>(weight)?;
////                let sign_repr = match sign {
////                    types::PgNumericSign::Positive => 0x0000,
////                    types::PgNumericSign::Negative => 0x4000,
////                };
////                self.inner.write_i16::<NetworkEndian>(sign_repr)?;;
////                self.inner.write_i16::<NetworkEndian>(0)?;
////
////                Ok(())
//            }
//        }
