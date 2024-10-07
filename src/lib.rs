//! Interchange File Format parser.

// TODO: there are some core chunks in all IFF (FORM, LIST, CAT, also revision numbers)
// see https://www.fileformat.info/format/iff/egff.htm

use chunk_parser::prelude::*;

use std::io::{Read, Seek};

//------------------------------------------------------------------------------

/// IFF parser implementation.
#[chunk_parser]
pub struct IFFParser;

/// IFF header layout.
pub struct IFFHeader {
    typeid: TypeId,
    size: u32
}

/// IFF header parser.
impl<R: Read> HeaderParser<IFFHeader> for IFFParser<R> {
    fn header(&mut self) -> chunk_parser::Result<IFFHeader> {
        Ok( IFFHeader { typeid: self.read()?, size: self.read_be()? })
    }
}

impl<R: Read + Seek> IFFParser<R> {
    /// Heuristic parser for nested IFF file structures.
    pub fn heuristic(&mut self, &IFFHeader { typeid, size }: &IFFHeader) -> chunk_parser::Result<u64> {
        let pos = self.position()?;
        let depth = self.depth();

        println!(
            "{:#08} {}{} {}{: >16} bytes", self.position()? - 8,
            " ".repeat(depth as usize * 2), FourCC(typeid),
            " ".repeat(16 - depth as usize * 2), size
        );

        // heuristically identify nested groups
        if size >= 12 { // need at least 12 bytes for a group
            let subid = FourCC(self.read::<TypeId>()?); // if the next 4 bytes are a valid fourcc, it could be a container like FORM
            let next_id = FourCC(self.read::<TypeId>()?); // the next 8 bytes will need to be a valid chunk header
            let next_len: u32 = self.read_be()?; // the size can be validated too

            if subid.is_valid() && next_id.is_valid() && next_len < size - 8 { // presume to be a FORM-like list of subchunks
                println!("\x1B[A\x1B[{}C-> {}", 14 + depth * 2, subid);
                self.rewind(8)?;
                if let Err(_) = self.subchunks(IFFParser::heuristic, size as u64 - 4) { // rewind the parser on error
                    println!("\x1B[A\x1B[{}C       ", 14 + depth * 2);
                    print!("\x1B[0G");
                    self.seek(pos + size as u64)?;
                }
            } else { // unknown chunk, the only thing left to do is skip
                self.skip(size as u64 - 12)?;
            }
        } else { // chunk too small to contain a group
            self.skip(size as u64)?;
        }

        // iff aligns chunks to even offsets
        if size % 2 == 0 { Ok(size as u64) }
        else { self.skip(1)?; Ok(size as u64 + 1) }
    }
}

//------------------------------------------------------------------------------

pub mod prelude {
    pub use chunk_parser::prelude::*;
    pub use super::{IFFParser, IFFHeader};
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
        let mut iff = IFFParser::cursor(DATA);
        iff.parse(|parser, &IFFHeader { typeid, size }| {
            assert_eq!(parser.depth(), 0);
            match &typeid {
                b"FORM" => parser.skip(size as u64),
                b"TEST" => parser.skip(size as u64),
                _ => Err(chunk_parser::Error::ParseError)
            }
        }).unwrap();
    }
}
