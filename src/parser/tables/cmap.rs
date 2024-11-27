use nom::combinator::map;
use nom::multi::count;
use nom::number::complete::{be_i16, be_u16, be_u32};
use nom::sequence::{pair, tuple};
use nom::IResult;

use crate::parser::RawFontData;
use crate::tag::Tag;

#[derive(Debug)]
pub struct CmapHeader {
    version: u16,
    num_tables: u16,
}

#[derive(Debug)]
pub struct CmapEncodingRecord {
    platform: u16,
    encoding: u16,
    offset: u32,
}

#[derive(Debug)]
pub enum Format<'a> {
    Format4(Format4<'a>),
    Unimplemented,
}

#[derive(Debug)]
pub struct Format4<'a> {
    format: u16,
    length: u16,
    language: u16,
    seg_count_x2: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    end_code: Vec<u16>,
    reserved_pad: u16,
    start_code: Vec<u16>,
    id_delta: Vec<i16>,
    id_range_offset: Vec<u16>,
    id_range_base: &'a [u8],
}

impl<'a> Format4<'a> {
    pub fn get_glyph_id(&self, c: usize) -> Option<usize> {
        let mut i = None;
        for (index, code) in self.end_code.iter().enumerate() {
            let code = (*code) as usize;
            if code >= c {
                i = Some(index);
                break;
            }
        }
        if let Some(i) = i {
            let start = self.start_code[i] as usize;
            assert!(c >= start);
            let offset = self.id_range_offset[i] as usize;
            let delta = self.id_delta[i] as isize;
            if offset == 0 {
                return Some((delta + c as isize) as usize);
            }
            let sum = (c - start) + offset / 2;
            let glyph_id_data = &self.id_range_base[i * 2 + sum..];
            let glyph_id = u16::from_be_bytes([glyph_id_data[0], glyph_id_data[1]]) as usize;
            return Some(glyph_id);
        } else {
            None
        }
    }
}

impl<'a> Format<'a> {
    pub fn format4(
        format: u16,
        length: u16,
        language: u16,
        seg_count_x2: u16,
        search_range: u16,
        entry_selector: u16,
        range_shift: u16,
        end_code: Vec<u16>,
        reserved_pad: u16,
        start_code: Vec<u16>,
        id_delta: Vec<i16>,
        id_range_offset: Vec<u16>,
        id_range_base: &'a [u8],
    ) -> Self {
        Format::Format4(Format4 {
            format,
            length,
            language,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            reserved_pad,
            start_code,
            id_delta,
            id_range_offset,
            id_range_base,
        })
    }

    pub fn get_glyph_id(&self, c: char) -> Option<usize> {
        let c = c as usize;
        match self {
            Format::Format4(format4) => format4.get_glyph_id(c),
            Format::Unimplemented => None,
        }
    }
}

impl<'a> RawFontData<'a> {
    pub fn read_cmap(&self) {
        let cmap = self.find_table_record(Tag::cmap);
        let beginning = &self.data[cmap.offset as usize..];
        let (data, header) = header(beginning).unwrap();
        let (data, records) = count(encoding_record, header.num_tables as usize)(data).unwrap();
        for record in records {
            println!("record: {record:?}");
            let (_, format) = read_format(&beginning[record.offset as usize..]).unwrap();
            if let Some(glyph_id) = format.get_glyph_id('A') {
                println!("glyph id of A is: {glyph_id}");
            }
            if let Some(glyph_id) = format.get_glyph_id('D') {
                println!("glyph id of D is: {glyph_id}");
            }
            if let Some(glyph_id) = format.get_glyph_id('a') {
                println!("glyph id of a is: {glyph_id}");
            }
        }
    }
}

fn read_format(input: &[u8]) -> IResult<&[u8], Format> {
    let (_, format) = be_u16(input)?;
    match format {
        4 => read_format4(input),
        _ => Ok((input, Format::Unimplemented)),
    }
}

fn read_format4(input: &[u8]) -> IResult<&[u8], Format> {
    let (data, (format, length, language, seg_count_x2, search_range, entry_selector, range_shift)) =
        tuple((be_u16, be_u16, be_u16, be_u16, be_u16, be_u16, be_u16))(input)?;
    let seg_count = seg_count_x2 / 2;
    let (data, end_code) = count(be_u16, seg_count as usize)(data)?;
    let (data, reserved_pad) = be_u16(data)?;
    assert!(reserved_pad == 0);
    let (data, start_code) = count(be_u16, seg_count as usize)(data)?;
    let (id_range_base, id_delta) = count(be_i16, seg_count as usize)(data)?;
    let (data, id_range_offset) = count(be_u16, seg_count as usize)(id_range_base)?;
    Ok((
        data,
        Format::format4(
            format,
            length,
            language,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            reserved_pad,
            start_code,
            id_delta,
            id_range_offset,
            id_range_base,
        ),
    ))
}

fn header(input: &[u8]) -> IResult<&[u8], CmapHeader> {
    map(pair(be_u16, be_u16), |(version, num_tables)| CmapHeader {
        version,
        num_tables,
    })(input)
}

fn encoding_record(input: &[u8]) -> IResult<&[u8], CmapEncodingRecord> {
    map(
        tuple((be_u16, be_u16, be_u32)),
        |(platform, encoding, offset)| CmapEncodingRecord {
            platform,
            encoding,
            offset,
        },
    )(input)
}
