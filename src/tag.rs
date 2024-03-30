#![allow(non_camel_case_types)]

#[macro_export]
macro_rules! build_tag {
    ($(($s:expr, $tag:ident)),*) => {
        #[derive(Debug)]
        pub enum Tag {
            $($tag,)*
        }

        impl From<String> for Tag {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $($s => Tag::$tag,)*
                    _ => unimplemented!("don't know how to process {} for now", value),
                }
            }
        }
    };
}

build_tag!(
    ("glyf", glyf),
    ("loca", loca),
    ("fpgm", fpgm),
    ("prep", prep),
    ("cvt ", cvt),
    ("gasp", gasp),
    ("head", head),
    ("hhea", hhea),
    ("hmtx", hmtx),
    ("maxp", maxp),
    ("morx", morx),
    ("name", name),
    ("post", post),
    ("prop", prop),
    ("vhea", vhea),
    ("vmtx", vmtx),
    ("FFTM", FFTM),
    ("GDEF", GDEF),
    ("GPOS", GPOS),
    ("GSUB", GSUB),
    ("OS/2", OS2),
    ("cmap", cmap),
    ("feat", feat),
    ("avar", avar),
    ("BASE", BASE),
    ("CBDT", CBDT),
    ("CBLC", CBLC)
);
