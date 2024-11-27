#![allow(non_camel_case_types)]

#[macro_export]
macro_rules! build_tag {
    ($(($s:expr, $tag:ident)),*) => {
        #[derive(Debug,PartialEq)]
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
    ("BASE", BASE),
    ("CBDT", CBDT),
    ("CBLC", CBLC),
    ("FFTM", FFTM),
    ("GDEF", GDEF),
    ("GPOS", GPOS),
    ("GSUB", GSUB),
    ("OS/2", OS2),
    ("avar", avar),
    ("cmap", cmap),
    ("cvt ", cvt),
    ("feat", feat),
    ("fpgm", fpgm),
    ("gasp", gasp),
    ("glyf", glyf),
    ("head", head),
    ("hhea", hhea),
    ("hmtx", hmtx),
    ("loca", loca),
    ("maxp", maxp),
    ("morx", morx),
    ("name", name),
    ("post", post),
    ("prep", prep),
    ("prop", prop),
    ("vhea", vhea),
    ("vmtx", vmtx)
);
