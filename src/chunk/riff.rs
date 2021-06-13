use crate::chunk::{ChunkHeader, FourCC};

#[derive(Clone, Copy, Debug)]
pub struct RiffChunk {
    pub chunk_header: ChunkHeader,
    pub form_type: FourCC,
}

impl RiffChunk {
    fn new(size: u32) -> Self {
        Self {
            chunk_header: ChunkHeader::into_riff_chunk(size),
            form_type: FourCC::Riff,
        }
    }
}
