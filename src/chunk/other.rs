use crate::chunk::ChunkHeader;

#[derive(Clone, Debug)]
pub struct OtherChunk<'a> {
    pub chunk_header: ChunkHeader,
    pub data: &'a [u8],
}
