use crate::chunk::ChunkHeader;

#[derive(Clone, Debug)]
pub struct OtherChunk<'a> {
    pub chunk_header: ChunkHeader,
    pub data: &'a [u8],
}

impl<'a> OtherChunk<'a> {
    pub fn new(four_cc: [u8; 4], size: u32, data: &'a [u8]) -> Self {
        Self {
            chunk_header: ChunkHeader::into_other_chunk(four_cc, size),
            data,
        }
    }
}
