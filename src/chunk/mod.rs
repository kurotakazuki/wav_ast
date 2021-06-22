pub use self::data::DataChunk;
pub use self::fmt::{FmtChunk, FormatTag, WaveFormatExtensible};
pub use self::other::OtherChunk;
pub use self::riff::RiffChunk;

mod data;
mod fmt;
mod other;
mod riff;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FourCC {
    Riff,
    Wave,
    Fmt,
    Data,
    Other([u8; 4]),
}

impl From<[u8; 4]> for FourCC {
    fn from(bytes: [u8; 4]) -> Self {
        match &bytes {
            b"RIFF" => FourCC::Riff,
            b"WAVE" => FourCC::Wave,
            b"fmt " => FourCC::Fmt,
            b"data" => FourCC::Data,
            _ => FourCC::Other(bytes),
        }
    }
}

impl From<FourCC> for [u8; 4] {
    fn from(four_cc: FourCC) -> Self {
        // TODO Whether change into FourCC?
        match four_cc {
            FourCC::Riff => [0x52, 0x49, 0x46, 0x46],
            FourCC::Wave => [0x57, 0x41, 0x56, 0x45],
            FourCC::Fmt => [0x66, 0x6D, 0x74, 0x20],
            FourCC::Data => [0x64, 0x61, 0x74, 0x61],
            FourCC::Other(bytes) => bytes,
        }
    }
}

impl From<&FourCC> for [u8; 4] {
    fn from(four_cc: &FourCC) -> Self {
        // TODO Whether change into FourCC?
        match four_cc {
            FourCC::Riff => [0x52, 0x49, 0x46, 0x46],
            FourCC::Wave => [0x57, 0x41, 0x56, 0x45],
            FourCC::Fmt => [0x66, 0x6D, 0x74, 0x20],
            FourCC::Data => [0x64, 0x61, 0x74, 0x61],
            FourCC::Other(bytes) => *bytes,
        }
    }
}

impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 4] = self.into();
        write!(f, "{}", std::str::from_utf8(&bytes).unwrap())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkHeader {
    pub id: FourCC,
    pub size: u32,
}

impl ChunkHeader {
    pub fn into_data_chunk(size: u32) -> ChunkHeader {
        let id = FourCC::Data;
        Self { id, size }
    }

    pub fn into_fmt_chunk(size: u32) -> ChunkHeader {
        let id = FourCC::Fmt;
        Self { id, size }
    }

    pub fn into_other_chunk(name: [u8; 4], size: u32) -> ChunkHeader {
        let id = FourCC::Other(name);
        Self { id, size }
    }

    pub fn into_riff_chunk(size: u32) -> ChunkHeader {
        let id = FourCC::Riff;
        Self { id, size }
    }
}

#[cfg(test)]
mod tests {
    use super::FourCC;

    // TODO test all patterns by macro

    #[test]
    fn four_cc_from_array() {
        let riff: &[u8; 4] = b"RIFF".into();
        let wave: &[u8; 4] = b"WAVE".into();
        let fmt: &[u8; 4] = b"fmt ".into();
        let data: &[u8; 4] = b"data".into();
        let other: &[u8; 4] = b"else".into();

        assert_eq!(FourCC::Riff, FourCC::from(*riff));
        assert_eq!(FourCC::Wave, FourCC::from(*wave));
        assert_eq!(FourCC::Fmt, FourCC::from(*fmt));
        assert_eq!(FourCC::Data, FourCC::from(*data));
        assert_eq!(
            FourCC::Other([0x65, 0x6C, 0x73, 0x65]),
            FourCC::from(*other)
        );
    }

    #[test]
    fn array_from_four_cc() {
        let self_riff: [u8; 4] = FourCC::Riff.into();
        let self_wave: [u8; 4] = FourCC::Wave.into();
        let self_fmt: [u8; 4] = FourCC::Fmt.into();
        let self_data: [u8; 4] = FourCC::Data.into();
        let self_other: [u8; 4] = FourCC::Other([0x65, 0x6C, 0x73, 0x65]).into();

        let riff: &[u8; 4] = b"RIFF".into();
        let wave: &[u8; 4] = b"WAVE".into();
        let fmt: &[u8; 4] = b"fmt ".into();
        let data: &[u8; 4] = b"data".into();
        let other: &[u8; 4] = b"else".into();

        assert_eq!(self_riff, *riff);
        assert_eq!(self_wave, *wave);
        assert_eq!(self_fmt, *fmt);
        assert_eq!(self_data, *data);
        assert_eq!(self_other, *other);
    }
}
