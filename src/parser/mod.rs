use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    multi::count,
    number::complete::{be_u16, be_u32},
    sequence::Tuple,
    IResult, Parser,
};

use crate::tag::Tag;

mod primitive_type;

#[derive(Debug)]
pub struct RawFontData<'a> {
    pub data: &'a [u8],
    pub tables: Vec<TableRecord>,
}

#[derive(Debug)]
pub struct TableRecord {
    tag: Tag,
    checksum: u32,
    offset: u32,
    length: u32,
}

pub fn ttf_parser(input: &[u8]) -> IResult<&[u8], RawFontData> {
    let origin_input = input;
    let sfnt_version = alt((
        tag(0x00010000u32.to_be_bytes()),
        tag(0x4F54544Fu32.to_be_bytes()),
    ));
    let table_num = be_u16;
    let search_range = be_u16;
    let entry_selector = be_u16;
    let range_shift = be_u16;
    let (input, (_, table_num, _, _, _)) = (
        sfnt_version,
        table_num,
        search_range,
        entry_selector,
        range_shift,
    )
        .parse(input)?;

    let (input, tables) = count(table_record_parser, table_num as usize)(input)?;

    // println!("table_num: {table_num}, search_range: {search_range}, entry_selector: {entry_selector}, range_shift: {range_shift}");

    Ok((
        input,
        RawFontData {
            data: origin_input,
            tables,
        },
    ))
}

pub fn table_record_parser(input: &[u8]) -> IResult<&[u8], TableRecord> {
    // refer to https://learn.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-truetype-outlines
    let table_tag = take(4usize).map(|n: &[u8]| {
        let res: String = n.iter().map(|n| *n as char).collect();
        res
    });
    let check_sum = be_u32;
    let offset = be_u32;
    let length = be_u32;

    let (input, (tag, checksum, offset, length)) =
        (table_tag, check_sum, offset, length).parse(input)?;
    Ok((
        input,
        TableRecord {
            tag: tag.into(),
            checksum,
            offset,
            length,
        },
    ))
}
