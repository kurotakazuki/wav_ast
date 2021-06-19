use crate::chunk::ChunkHeader;

#[derive(Clone, Copy, Debug)]
pub struct FactChunk {
    pub chunk_header: ChunkHeader,
    pub sample_length: u32,
}

impl FactChunk {
    pub fn new(size: u32, sample_length: u32) -> Self {
        Self {
            chunk_header: ChunkHeader::into_fact_chunk(size),
            sample_length,
        }
    }
}
