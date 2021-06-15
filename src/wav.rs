use crate::chunk::{DataChunk, FactChunk, FmtChunk, OtherChunk, RiffChunk};

#[derive(Clone, Debug)]
pub struct Wav<'a> {
    riff: RiffChunk,
    fmt: FmtChunk,
    fact: Option<FactChunk>,
    others: Vec<OtherChunk<'a>>,
    data: DataChunk,
}
