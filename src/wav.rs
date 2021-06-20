use crate::chunk::{DataChunk, FactChunk, FmtChunk, OtherChunk, RiffChunk};

#[derive(Clone, Debug)]
pub struct Wav<'a> {
    pub riff: RiffChunk,
    pub fmt: FmtChunk,
    pub fact: Option<FactChunk>,
    pub others: Vec<OtherChunk<'a>>,
    pub data: DataChunk,
}

impl<'a> Wav<'a> {
    pub fn new(
        riff: RiffChunk,
        fmt: FmtChunk,
        fact: Option<FactChunk>,
        others: Vec<OtherChunk<'a>>,
        data: DataChunk,
    ) -> Self {
        Self {
            riff,
            fmt,
            fact,
            others,
            data,
        }
    }
}
