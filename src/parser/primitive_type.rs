use std::ops::RangeFrom;

use nom::{
    error::ParseError,
    number::complete::{be_u16, be_u32},
    IResult, InputIter, InputLength, Parser, Slice,
};

#[inline]
pub fn be_f2dot14<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    be_u16
        .map(|x| {
            let is_neg = x & 0x8000 != 0;
            let is_one = x & 0x4000 != 0;
            let x = x & 0x3fff;
            ((x as f32) / 16384.0 + if is_one { 1.0 } else { 0.0 })
                * if is_neg { -1.0 } else { 1.0 }
        })
        .parse(input)
}

#[inline]
pub fn be_fixed<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
    I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    be_u32
        .map(|x| {
            let upper = ((x & 0xffff0000) >> 16) as u16;
            let lower = (x & 0xffff) as u16;
            let integer = upper as i16 as f32;
            let fraction = lower as f32 / 65536.0;
            integer + fraction
        })
        .parse(input)
}
