use crate::chunk::{ChunkHeader, FourCC};

#[derive(Clone, Copy, Debug)]
pub struct RiffChunk {
    pub chunk_header: ChunkHeader,
    pub form_type: FourCC,
}
