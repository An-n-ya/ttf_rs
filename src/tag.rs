#[allow(non_camel_case_types)]
pub enum Tag {
    glyf,
    loca,
    cvt,
    fpgm,
    prep,
    gasp,
    MTFF,
}

impl From<[u8; 4]> for Tag {
    fn from(value: [u8; 4]) -> Self {
        match &value {
            b"glyf" => Tag::glyf,
            b"loca" => Tag::loca,
            b"cvt " => Tag::cvt,
            b"fpgm" => Tag::fpgm,
            b"prep" => Tag::prep,
            b"gasp" => Tag::gasp,
            b"MTFF" => Tag::MTFF,
            _ => unimplemented!(
                "don't know how to process {} for now",
                String::from_utf8_lossy(&value)
            ),
        }
    }
}
impl From<&[u8]> for Tag {
    fn from(value: &[u8]) -> Self {
        match value {
            b"glyf" => Tag::glyf,
            b"loca" => Tag::loca,
            b"cvt " => Tag::cvt,
            b"fpgm" => Tag::fpgm,
            b"prep" => Tag::prep,
            b"gasp" => Tag::gasp,
            b"MTFF" => Tag::MTFF,
            _ => unimplemented!(
                "don't know how to process {} for now",
                String::from_utf8_lossy(value)
            ),
        }
    }
}
