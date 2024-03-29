use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::count,
    number::complete::{be_u16, be_u32},
    sequence::Tuple,
    IResult, Parser,
};

use crate::tag::Tag;

mod primitive_type;

pub struct RawFontData {
    tables: Vec<TableRecord>,
}
pub struct TableRecord {
    tag: Tag,
    checksum: u32,
    offset: u32,
    length: u32,
}

pub fn ttf_parser(input: &[u8]) -> IResult<&[u8], RawFontData> {
    let sfnt_version = alt((
        tag(0x00010000u32.to_be_bytes()),
        tag(0x4F54544Fu32.to_be_bytes()),
    ));
    let table_num = be_u16;
    let search_range = be_u16;
    let entry_selector = be_u16;
    let range_shift = be_u16;
    let (input, (_, table_num, search_range, entry_selector, range_shift)) = (
        sfnt_version,
        table_num,
        search_range,
        entry_selector,
        range_shift,
    )
        .parse(input)?;

    let (input, tables) = count(table_record_parser, table_num as usize)(input)?;

    // println!("table_num: {table_num}, search_range: {search_range}, entry_selector: {entry_selector}, range_shift: {range_shift}");

    Ok((input, RawFontData { tables }))
}

pub fn table_record_parser(input: &[u8]) -> IResult<&[u8], TableRecord> {
    // refer to https://learn.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-truetype-outlines
    // let table_tag = alt((
    //     tag(b"glyf"),
    //     tag(b"loca"),
    //     tag(b"cvt "),
    //     tag(b"fpgm"),
    //     tag(b"prep"),
    //     tag(b"gasp"),
    // ));
    let table_tag = be_u32;
    let check_sum = be_u32;
    let offset = be_u32;
    let length = be_u32;

    let (input, (tag, checksum, offset, length)) =
        (table_tag, check_sum, offset, length).parse(input)?;
    Ok((
        input,
        TableRecord {
            tag: tag.to_le_bytes().into(),
            checksum,
            offset,
            length,
        },
    ))
}
