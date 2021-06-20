use crate::chunk::ChunkHeader;
use crate::sample::Sample;

#[derive(Clone, Debug)]
pub struct DataChunk {
    pub chunk_header: ChunkHeader,
    pub data: Vec<Vec<Sample>>,
}

impl DataChunk {
    pub fn new(size: u32, data: Vec<Vec<Sample>>) -> Self {
        Self {
            chunk_header: ChunkHeader::into_data_chunk(size),
            data,
        }
    }
}
