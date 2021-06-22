use crate::variable::{WavVariable, WavVariable::*};
use mpl::choice::{First, Second};
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{Metasymbol::*, TerminalSymbol, U8SliceTerminal, U8SliceTerminal::*, E};

pub struct WavRules;

impl<'a> WavRules {
    const WAV_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Riff),
            rhs: E::V(ChunksAndData),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const CHUNKS_AND_DATA_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Chunks),
            rhs: E::V(Data),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const CHUNKS_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Chunk),
            rhs: E::V(Chunks),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Epsilon))),
    };

    const CHUNK_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Fmt),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::V(Other)),
    };

    // Riff Chunk
    const RIFF_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("RIFF"))),
            rhs: E::V(FileSize),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const FILE_SIZE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U32),
            rhs: E::T(TerminalSymbol::Original(Str("WAVE"))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Fmt Chunk
    const FMT_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("fmt "))),
            rhs: E::V(FmtSize),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const FMT_SIZE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(16))),
            rhs: E::V(FormatTag),
        },
        second: Second(E::V(FmtExt)),
    };
    const FORMAT_TAG_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::V(Channels),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const CHANNELS_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::V(SamplesPerSec),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const SAMPLES_PER_SEC_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U32),
            rhs: E::V(AvgBytesPerSec),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const AVG_BYTES_PER_SEC_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U32),
            rhs: E::V(BlockAlign),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const BLOCK_ALIGN_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::V(BitsPerSample),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const BITS_PER_SAMPLE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    const FMT_EXT_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(40))),
            rhs: E::V(FormatTagWaveFormatExtensible),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const FORMAT_TAG_WAVE_FORMAT_EXTENSIBLE_RIGHT_RULE: RightRule<
        U8SliceTerminal<'a>,
        WavVariable,
    > = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU16(0xFFFE))),
            rhs: E::V(WaveFormatExtensible),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const WAVE_FORMAT_EXTENSIBLE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> =
        RightRule {
            first: First {
                lhs: E::V(Channels),
                rhs: E::V(CbSize),
            },
            second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
        };
    const CB_SIZE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU16(22))),
            rhs: E::V(ValidBitsPerSample),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const VALID_BITS_PER_SAMPLE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> =
        RightRule {
            first: First {
                lhs: E::V(U16),
                rhs: E::V(SamplesPerBlock),
            },
            second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
        };
    const SAMPLES_PER_BLOCK_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::V(Reserved),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const RESERVED_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U16),
            rhs: E::V(ChannelMask),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const CHANNEL_MASK_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U32),
            rhs: E::V(SubFormat),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const SUB_FORMAT_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U128),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Other Chunk
    const OTHER_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(4))),
            rhs: E::V(OtherSize1),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const OTHER_SIZE1_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1))),
        },
        second: Second(E::V(OtherSize4)),
    };
    const OTHER_SIZE4_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(4))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(4))),
        },
        second: Second(E::V(OtherSize24)),
    };
    const OTHER_SIZE24_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(24))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(24))),
        },
        second: Second(E::V(OtherSize28)),
    };
    const OTHER_SIZE28_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(28))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(28))),
        },
        second: Second(E::V(OtherSize602)),
    };
    const OTHER_SIZE602_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(602))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(602))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Data Chunk
    const DATA_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("data"))),
            rhs: E::V(DataSize),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const DATA_SIZE_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(U32),
            rhs: E::T(TerminalSymbol::Metasymbol(All)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    const U16_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(2))),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const U32_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(4))),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const U128_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(8))),
            rhs: E::T(TerminalSymbol::Metasymbol(Epsilon)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
}

impl<'a> Rules<U8SliceTerminal<'a>, WavVariable> for WavRules {
    fn get(&self, variable: &WavVariable) -> Option<&RightRule<U8SliceTerminal<'a>, WavVariable>> {
        Some(match variable {
            Wav => &Self::WAV_RIGHT_RULE,
            ChunksAndData => &Self::CHUNKS_AND_DATA_RIGHT_RULE,
            Chunks => &Self::CHUNKS_RIGHT_RULE,

            Chunk => &Self::CHUNK_RIGHT_RULE,

            // Riff Chunk
            Riff => &Self::RIFF_RIGHT_RULE,
            FileSize => &Self::FILE_SIZE_RIGHT_RULE,

            // Fmt Chunk
            Fmt => &Self::FMT_RIGHT_RULE,
            FmtSize => &Self::FMT_SIZE_RIGHT_RULE,
            FormatTag => &Self::FORMAT_TAG_RIGHT_RULE,
            Channels => &Self::CHANNELS_RIGHT_RULE,
            SamplesPerSec => &Self::SAMPLES_PER_SEC_RIGHT_RULE,
            AvgBytesPerSec => &Self::AVG_BYTES_PER_SEC_RIGHT_RULE,
            BlockAlign => &Self::BLOCK_ALIGN_RIGHT_RULE,
            BitsPerSample => &Self::BITS_PER_SAMPLE_RIGHT_RULE,

            FmtExt => &Self::FMT_EXT_RIGHT_RULE,
            FormatTagWaveFormatExtensible => &Self::FORMAT_TAG_WAVE_FORMAT_EXTENSIBLE_RIGHT_RULE,
            WaveFormatExtensible => &Self::WAVE_FORMAT_EXTENSIBLE_RIGHT_RULE,
            CbSize => &Self::CB_SIZE_RIGHT_RULE,
            ValidBitsPerSample => &Self::VALID_BITS_PER_SAMPLE_RIGHT_RULE,
            SamplesPerBlock => &Self::SAMPLES_PER_BLOCK_RIGHT_RULE,
            Reserved => &Self::RESERVED_RIGHT_RULE,
            ChannelMask => &Self::CHANNEL_MASK_RIGHT_RULE,
            SubFormat => &Self::SUB_FORMAT_RIGHT_RULE,

            // Other Chunk
            Other => &Self::OTHER_RIGHT_RULE,
            OtherSize1 => &Self::OTHER_SIZE1_RIGHT_RULE,
            OtherSize4 => &Self::OTHER_SIZE4_RIGHT_RULE,
            OtherSize24 => &Self::OTHER_SIZE24_RIGHT_RULE,
            OtherSize28 => &Self::OTHER_SIZE28_RIGHT_RULE,
            OtherSize602 => &Self::OTHER_SIZE602_RIGHT_RULE,

            // Data Chunk
            Data => &Self::DATA_RIGHT_RULE,
            DataSize => &Self::DATA_SIZE_RIGHT_RULE,

            U16 => &Self::U16_RIGHT_RULE,
            U32 => &Self::U32_RIGHT_RULE,
            U128 => &Self::U128_RIGHT_RULE,

            _ => unimplemented!(),
        })
    }
}
