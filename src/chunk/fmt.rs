use crate::chunk::ChunkHeader;

#[derive(Clone, Copy, Debug)]
pub enum FormatTag {
    UncompressedPCM,
    IEEEFloatingPoint,
    WaveFormatExtensible,
    Other(u16),
}

impl Default for FormatTag {
    fn default() -> Self {
        FormatTag::Other(0)
    }
}

impl From<u16> for FormatTag {
    fn from(n: u16) -> Self {
        match n {
            1 => FormatTag::UncompressedPCM,
            3 => FormatTag::IEEEFloatingPoint,
            65534 => FormatTag::WaveFormatExtensible,
            _ => FormatTag::Other(n),
        }
    }
}

impl From<FormatTag> for u16 {
    fn from(format_tag: FormatTag) -> Self {
        match format_tag {
            FormatTag::UncompressedPCM => 1,
            FormatTag::IEEEFloatingPoint => 3,
            FormatTag::WaveFormatExtensible => 65534,
            FormatTag::Other(n) => n,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WaveFormatExtensible {
    pub valid_bits_per_sample: u16,
    pub samples_per_block: u16,
    pub reserved: u16,
    pub channel_mask: u32,
    pub sub_format: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct FmtChunk {
    pub chunk_header: ChunkHeader,
    /// Format Tag
    pub format_tag: FormatTag,
    /// Channels
    pub channels: u16,
    /// Samples Per Sec
    pub samples_per_sec: u32,
    /// Avg Bytes Per Sec
    pub avg_bytes_per_sec: u32,
    /// Block Align
    pub block_align: u16,
    /// Bits Per Sample
    pub bits_per_sample: u16,
    /// cbSize
    pub cb_size: Option<u16>,
    /// WaveFormatExtensible
    pub wave_format_extensible: Option<WaveFormatExtensible>,
}

impl FmtChunk {
    pub fn avg_bytes_per_sec(channels: u16, samples_per_sec: u32, bits_per_sample: u16) -> u32 {
        samples_per_sec * (channels * bits_per_sample) as u32 / 8
    }

    pub fn block_align(channels: u16, bits_per_sample: u16) -> u16 {
        channels * bits_per_sample / 8
    }
}
