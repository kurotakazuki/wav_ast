use crate::chunk::{DataChunk, FmtChunk, OtherChunk, RiffChunk};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Wav<'a> {
    pub riff: RiffChunk,
    pub fmt: FmtChunk,
    pub others: Vec<OtherChunk<'a>>,
    pub data: DataChunk,
}

impl<'a> Wav<'a> {
    pub fn new(
        riff: RiffChunk,
        fmt: FmtChunk,
        others: Vec<OtherChunk<'a>>,
        data: DataChunk,
    ) -> Self {
        Self {
            riff,
            fmt,
            others,
            data,
        }
    }

    pub fn file_size(&self) -> usize {
        self.riff.chunk_header.size as usize + 8
    }
}

impl fmt::Display for Wav<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File Size: {}", self.file_size())?;
        writeln!(f, "----------------------------------------")?;
        // writeln!(f, "Fmt Chunk")?;
        writeln!(f, "Format Tag: {:?}", self.fmt.format_tag)?;
        writeln!(f, "Channels: {}", self.fmt.channels)?;
        writeln!(f, "Samples Per Sec: {}", self.fmt.samples_per_sec)?;
        writeln!(f, "Avg Bytes Per Sec: {}", self.fmt.avg_bytes_per_sec)?;
        writeln!(f, "Block Align: {}", self.fmt.block_align)?;
        writeln!(f, "Bits Per Sample: {}", self.fmt.bits_per_sample)?;
        writeln!(f, "----------------------------------------")?;
        for other in &self.others {
            writeln!(f, "Other Chunk: {}", other.chunk_header.id)?;
        }
        writeln!(f, "----------------------------------------")?;
        writeln!(f, "Data Size: {}", self.data.chunk_header.size)?;
        writeln!(
            f,
            "Sample Frames: {}",
            self.fmt.sample_frames(self.data.chunk_header.size)
        )
    }
}
