//! Interchange File Format parser.

// TODO: there are some core chunks in all IFF (FORM, LIST, CAT, also revision numbers)
// see https://www.fileformat.info/format/iff/egff.htm

use chunk_parser::prelude::*;

//------------------------------------------------------------------------------

#[chunk_parser]
pub struct IFFParser;

/// IFF parser implementation.
impl<R> Parser for IFFParser<R> where R: std::io::Read + std::io::Seek {
    type Header = (TypeId, i32);
    type Size = i32;

    fn read_header(&mut self) -> chunk_parser::Result<Self::Header> {
        Ok((self.read()?, self.read_be()?))
    }

    fn guesser(&mut self, ( typeid, size ): &Self::Header) -> chunk_parser::Result<i32> {
        let pos = self.position()?;
        let depth = self.depth();

        println!(
            "{:#08} {}{} {}{: >16} bytes", self.position()? - 8,
            " ".repeat(depth * 2), FourCC(*typeid),
            " ".repeat(16 - depth * 2), size
        );

        // if the next 4 bytes are a valid fourcc, it could be a container like FORM
        let subid = self.read::<TypeId>()?;
        let container = FourCC(subid).is_valid() // the next 8 bytes will need to be a valid header also
                     && FourCC(self.peek::<TypeId>()?).is_valid();
                     //&& self.peek::<TypeId>(4)? < size - 8

        if container {
            println!("\x1B[A\x1B[{}C-> {}", 14 + depth * 2, FourCC(subid));
            // presume to be a FORM-like list of subchunks
            if let Err(_) = self.parse_subchunks(IFFParser::guesser, size - 4) {
                // rewind the parser on error
                println!("\x1B[A\x1B[{}C       ", 14 + depth * 2);
                print!("\x1B[0G");
                self.seek(pos + *size)?;
            }
        } else {
            // unknown chunk, the only thing left to do is skip
            self.skip(size - 4)?;
        }

        // iff aligns chunks to even offsets
        if size % 2 == 0 { Ok(*size) }
        else { self.skip(1)?; Ok(*size + 1) }
    }
}

//------------------------------------------------------------------------------

pub mod prelude {
    pub use chunk_parser::prelude::*;
    pub use super::IFFParser;
}

//==============================================================================

#[cfg(test)]
mod tests {
    use super::prelude::*;

    // nonsense data to test basic functionality
    const DATA: &[u8;24] = &[
        // FORM chunk (24 bytes)
        0x46, 0x4f, 0x52, 0x4d, // "FORM" chunk typeid
        0x00, 0x00, 0x00, 0x10, // Chunk size (16 bytes)
        0x54, 0x45, 0x53, 0x54, // Subchunk typeid ("TEST")

        // TEST chunk (12 bytes)
        0x54, 0x45, 0x53, 0x54, // "TEST" chunk typid
        0x00, 0x00, 0x00, 0x04, // Chunk size (4 bytes)
        0x01, 0x02, 0x03, 0x04, // Test data
    ];

    #[test]
    fn iff() {
        let mut iff = IFFParser::buf(DATA);
        iff.parse(|parser, ( typeid, size )| {
            assert_eq!(parser.depth(), 0);
            match typeid {
                b"FORM" => parser.expect(b"TEST")?.skip(size - 4),
                b"TEST" => parser.skip(*size),
                _ => Err(chunk_parser::Error::ParseError)
            }?;
            Ok(*size)
        }).unwrap();
    }
}
