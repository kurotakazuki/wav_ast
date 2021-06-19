use crate::chunk::ChunkHeader;

#[derive(Copy, Clone, Debug)]
pub enum Sample {
    U8(u8),
    U16(u16),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Debug)]
pub struct DataChunk {
    pub chunk_header: ChunkHeader,
    pub data: Vec<Vec<Sample>>,
}
