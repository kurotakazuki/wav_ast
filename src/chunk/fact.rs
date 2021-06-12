use crate::chunk::ChunkHeader;

#[derive(Clone, Copy, Debug)]
pub struct FactChunk {
    pub chunk_header: ChunkHeader,
    pub sample_length: u32,
}
