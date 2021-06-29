use crate::variable::{WavVariable, WavVariable::*};
use mpl::choices::{First, Second};
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{Metasymbol::*, TerminalSymbol, U8SliceTerminal, U8SliceTerminal::*, E};

pub struct WavRules;

impl<'a> WavRules {
    const WAV_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Riff),
            rhs: E::V(Chunks),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const CHUNKS_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Chunk),
            rhs: E::V(Chunks),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };

    const CHUNK_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Fmt),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Chunk2)),
    };
    const CHUNK2_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::V(Data),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
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
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
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
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
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
        second: Second(E::V(OtherSize2)),
    };
    const OTHER_SIZE2_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(2))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(2))),
        },
        second: Second(E::V(OtherSize3)),
    };
    const OTHER_SIZE3_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(3))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(3))),
        },
        second: Second(E::V(OtherSize4)),
    };
    const OTHER_SIZE4_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(4))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(4))),
        },
        second: Second(E::V(OtherSize5)),
    };
    const OTHER_SIZE5_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(5))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(5))),
        },
        second: Second(E::V(OtherSize6)),
    };
    const OTHER_SIZE6_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(6))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(6))),
        },
        second: Second(E::V(OtherSize7)),
    };
    const OTHER_SIZE7_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(7))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(7))),
        },
        second: Second(E::V(OtherSize8)),
    };
    const OTHER_SIZE8_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(8))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(8))),
        },
        second: Second(E::V(OtherSize9)),
    };
    const OTHER_SIZE9_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(9))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(9))),
        },
        second: Second(E::V(OtherSize10)),
    };
    const OTHER_SIZE10_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(10))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(10))),
        },
        second: Second(E::V(OtherSize11)),
    };
    const OTHER_SIZE11_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(11))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(11))),
        },
        second: Second(E::V(OtherSize12)),
    };
    const OTHER_SIZE12_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(12))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(12))),
        },
        second: Second(E::V(OtherSize13)),
    };
    const OTHER_SIZE13_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(13))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(13))),
        },
        second: Second(E::V(OtherSize14)),
    };
    const OTHER_SIZE14_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(14))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(14))),
        },
        second: Second(E::V(OtherSize15)),
    };
    const OTHER_SIZE15_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(15))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(15))),
        },
        second: Second(E::V(OtherSize16)),
    };
    const OTHER_SIZE16_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(16))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(16))),
        },
        second: Second(E::V(OtherSize17)),
    };
    const OTHER_SIZE17_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(17))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(17))),
        },
        second: Second(E::V(OtherSize18)),
    };
    const OTHER_SIZE18_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(18))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(18))),
        },
        second: Second(E::V(OtherSize19)),
    };
    const OTHER_SIZE19_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(19))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(19))),
        },
        second: Second(E::V(OtherSize20)),
    };
    const OTHER_SIZE20_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(20))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(20))),
        },
        second: Second(E::V(OtherSize21)),
    };
    const OTHER_SIZE21_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(21))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(21))),
        },
        second: Second(E::V(OtherSize22)),
    };
    const OTHER_SIZE22_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(22))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(22))),
        },
        second: Second(E::V(OtherSize23)),
    };
    const OTHER_SIZE23_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(23))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(23))),
        },
        second: Second(E::V(OtherSize24)),
    };
    const OTHER_SIZE24_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(24))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(24))),
        },
        second: Second(E::V(OtherSize25)),
    };
    const OTHER_SIZE25_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(25))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(25))),
        },
        second: Second(E::V(OtherSize26)),
    };
    const OTHER_SIZE26_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(26))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(26))),
        },
        second: Second(E::V(OtherSize27)),
    };
    const OTHER_SIZE27_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(27))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(27))),
        },
        second: Second(E::V(OtherSize28)),
    };
    const OTHER_SIZE28_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(28))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(28))),
        },
        second: Second(E::V(OtherSize29)),
    };
    const OTHER_SIZE29_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(29))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(29))),
        },
        second: Second(E::V(OtherSize30)),
    };
    const OTHER_SIZE30_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(30))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(30))),
        },
        second: Second(E::V(OtherSize31)),
    };
    const OTHER_SIZE31_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(31))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(31))),
        },
        second: Second(E::V(OtherSize32)),
    };
    const OTHER_SIZE32_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(32))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(32))),
        },
        second: Second(E::V(OtherSize33)),
    };
    const OTHER_SIZE33_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(33))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(33))),
        },
        second: Second(E::V(OtherSize34)),
    };
    const OTHER_SIZE34_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(34))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(34))),
        },
        second: Second(E::V(OtherSize35)),
    };
    const OTHER_SIZE35_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(35))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(35))),
        },
        second: Second(E::V(OtherSize36)),
    };
    const OTHER_SIZE36_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(36))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(36))),
        },
        second: Second(E::V(OtherSize37)),
    };
    const OTHER_SIZE37_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(37))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(37))),
        },
        second: Second(E::V(OtherSize38)),
    };
    const OTHER_SIZE38_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(38))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(38))),
        },
        second: Second(E::V(OtherSize39)),
    };
    const OTHER_SIZE39_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(39))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(39))),
        },
        second: Second(E::V(OtherSize40)),
    };
    const OTHER_SIZE40_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(40))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(40))),
        },
        second: Second(E::V(OtherSize41)),
    };
    const OTHER_SIZE41_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(41))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(41))),
        },
        second: Second(E::V(OtherSize42)),
    };
    const OTHER_SIZE42_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(42))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(42))),
        },
        second: Second(E::V(OtherSize43)),
    };
    const OTHER_SIZE43_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(43))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(43))),
        },
        second: Second(E::V(OtherSize44)),
    };
    const OTHER_SIZE44_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(44))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(44))),
        },
        second: Second(E::V(OtherSize45)),
    };
    const OTHER_SIZE45_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(45))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(45))),
        },
        second: Second(E::V(OtherSize46)),
    };
    const OTHER_SIZE46_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(46))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(46))),
        },
        second: Second(E::V(OtherSize47)),
    };
    const OTHER_SIZE47_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(47))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(47))),
        },
        second: Second(E::V(OtherSize48)),
    };
    const OTHER_SIZE48_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(48))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(48))),
        },
        second: Second(E::V(OtherSize49)),
    };
    const OTHER_SIZE49_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(49))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(49))),
        },
        second: Second(E::V(OtherSize50)),
    };
    const OTHER_SIZE50_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(50))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(50))),
        },
        second: Second(E::V(OtherSize51)),
    };
    const OTHER_SIZE51_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(51))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(51))),
        },
        second: Second(E::V(OtherSize52)),
    };
    const OTHER_SIZE52_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(52))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(52))),
        },
        second: Second(E::V(OtherSize53)),
    };
    const OTHER_SIZE53_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(53))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(53))),
        },
        second: Second(E::V(OtherSize54)),
    };
    const OTHER_SIZE54_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(54))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(54))),
        },
        second: Second(E::V(OtherSize55)),
    };
    const OTHER_SIZE55_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(55))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(55))),
        },
        second: Second(E::V(OtherSize56)),
    };
    const OTHER_SIZE56_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(56))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(56))),
        },
        second: Second(E::V(OtherSize57)),
    };
    const OTHER_SIZE57_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(57))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(57))),
        },
        second: Second(E::V(OtherSize58)),
    };
    const OTHER_SIZE58_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(58))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(58))),
        },
        second: Second(E::V(OtherSize59)),
    };
    const OTHER_SIZE59_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(59))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(59))),
        },
        second: Second(E::V(OtherSize60)),
    };
    const OTHER_SIZE60_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(60))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(60))),
        },
        second: Second(E::V(OtherSize61)),
    };
    const OTHER_SIZE61_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(61))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(61))),
        },
        second: Second(E::V(OtherSize62)),
    };
    const OTHER_SIZE62_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(62))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(62))),
        },
        second: Second(E::V(OtherSize63)),
    };
    const OTHER_SIZE63_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(63))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(63))),
        },
        second: Second(E::V(OtherSize64)),
    };
    const OTHER_SIZE64_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(64))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(64))),
        },
        second: Second(E::V(OtherSize65)),
    };
    const OTHER_SIZE65_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(65))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(65))),
        },
        second: Second(E::V(OtherSize66)),
    };
    const OTHER_SIZE66_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(66))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(66))),
        },
        second: Second(E::V(OtherSize67)),
    };
    const OTHER_SIZE67_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(67))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(67))),
        },
        second: Second(E::V(OtherSize68)),
    };
    const OTHER_SIZE68_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(68))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(68))),
        },
        second: Second(E::V(OtherSize69)),
    };
    const OTHER_SIZE69_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(69))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(69))),
        },
        second: Second(E::V(OtherSize70)),
    };
    const OTHER_SIZE70_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(70))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(70))),
        },
        second: Second(E::V(OtherSize71)),
    };
    const OTHER_SIZE71_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(71))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(71))),
        },
        second: Second(E::V(OtherSize72)),
    };
    const OTHER_SIZE72_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(72))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(72))),
        },
        second: Second(E::V(OtherSize73)),
    };
    const OTHER_SIZE73_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(73))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(73))),
        },
        second: Second(E::V(OtherSize74)),
    };
    const OTHER_SIZE74_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(74))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(74))),
        },
        second: Second(E::V(OtherSize75)),
    };
    const OTHER_SIZE75_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(75))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(75))),
        },
        second: Second(E::V(OtherSize76)),
    };
    const OTHER_SIZE76_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(76))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(76))),
        },
        second: Second(E::V(OtherSize77)),
    };
    const OTHER_SIZE77_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(77))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(77))),
        },
        second: Second(E::V(OtherSize78)),
    };
    const OTHER_SIZE78_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(78))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(78))),
        },
        second: Second(E::V(OtherSize79)),
    };
    const OTHER_SIZE79_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(79))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(79))),
        },
        second: Second(E::V(OtherSize80)),
    };
    const OTHER_SIZE80_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(80))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(80))),
        },
        second: Second(E::V(OtherSize81)),
    };
    const OTHER_SIZE81_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(81))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(81))),
        },
        second: Second(E::V(OtherSize82)),
    };
    const OTHER_SIZE82_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(82))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(82))),
        },
        second: Second(E::V(OtherSize83)),
    };
    const OTHER_SIZE83_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(83))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(83))),
        },
        second: Second(E::V(OtherSize84)),
    };
    const OTHER_SIZE84_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(84))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(84))),
        },
        second: Second(E::V(OtherSize85)),
    };
    const OTHER_SIZE85_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(85))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(85))),
        },
        second: Second(E::V(OtherSize86)),
    };
    const OTHER_SIZE86_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(86))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(86))),
        },
        second: Second(E::V(OtherSize87)),
    };
    const OTHER_SIZE87_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(87))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(87))),
        },
        second: Second(E::V(OtherSize88)),
    };
    const OTHER_SIZE88_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(88))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(88))),
        },
        second: Second(E::V(OtherSize89)),
    };
    const OTHER_SIZE89_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(89))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(89))),
        },
        second: Second(E::V(OtherSize90)),
    };
    const OTHER_SIZE90_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(90))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(90))),
        },
        second: Second(E::V(OtherSize91)),
    };
    const OTHER_SIZE91_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(91))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(91))),
        },
        second: Second(E::V(OtherSize92)),
    };
    const OTHER_SIZE92_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(92))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(92))),
        },
        second: Second(E::V(OtherSize93)),
    };
    const OTHER_SIZE93_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(93))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(93))),
        },
        second: Second(E::V(OtherSize94)),
    };
    const OTHER_SIZE94_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(94))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(94))),
        },
        second: Second(E::V(OtherSize95)),
    };
    const OTHER_SIZE95_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(95))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(95))),
        },
        second: Second(E::V(OtherSize96)),
    };
    const OTHER_SIZE96_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(96))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(96))),
        },
        second: Second(E::V(OtherSize97)),
    };
    const OTHER_SIZE97_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(97))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(97))),
        },
        second: Second(E::V(OtherSize98)),
    };
    const OTHER_SIZE98_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(98))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(98))),
        },
        second: Second(E::V(OtherSize99)),
    };
    const OTHER_SIZE99_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(99))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(99))),
        },
        second: Second(E::V(OtherSize100)),
    };
    const OTHER_SIZE100_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(100))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(100))),
        },
        second: Second(E::V(OtherSize101)),
    };
    const OTHER_SIZE101_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(101))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(101))),
        },
        second: Second(E::V(OtherSize102)),
    };
    const OTHER_SIZE102_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(102))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(102))),
        },
        second: Second(E::V(OtherSize103)),
    };
    const OTHER_SIZE103_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(103))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(103))),
        },
        second: Second(E::V(OtherSize104)),
    };
    const OTHER_SIZE104_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(104))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(104))),
        },
        second: Second(E::V(OtherSize105)),
    };
    const OTHER_SIZE105_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(105))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(105))),
        },
        second: Second(E::V(OtherSize106)),
    };
    const OTHER_SIZE106_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(106))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(106))),
        },
        second: Second(E::V(OtherSize107)),
    };
    const OTHER_SIZE107_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(107))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(107))),
        },
        second: Second(E::V(OtherSize108)),
    };
    const OTHER_SIZE108_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(108))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(108))),
        },
        second: Second(E::V(OtherSize109)),
    };
    const OTHER_SIZE109_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(109))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(109))),
        },
        second: Second(E::V(OtherSize110)),
    };
    const OTHER_SIZE110_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(110))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(110))),
        },
        second: Second(E::V(OtherSize111)),
    };
    const OTHER_SIZE111_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(111))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(111))),
        },
        second: Second(E::V(OtherSize112)),
    };
    const OTHER_SIZE112_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(112))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(112))),
        },
        second: Second(E::V(OtherSize113)),
    };
    const OTHER_SIZE113_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(113))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(113))),
        },
        second: Second(E::V(OtherSize114)),
    };
    const OTHER_SIZE114_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(114))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(114))),
        },
        second: Second(E::V(OtherSize115)),
    };
    const OTHER_SIZE115_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(115))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(115))),
        },
        second: Second(E::V(OtherSize116)),
    };
    const OTHER_SIZE116_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(116))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(116))),
        },
        second: Second(E::V(OtherSize117)),
    };
    const OTHER_SIZE117_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(117))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(117))),
        },
        second: Second(E::V(OtherSize118)),
    };
    const OTHER_SIZE118_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(118))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(118))),
        },
        second: Second(E::V(OtherSize119)),
    };
    const OTHER_SIZE119_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(119))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(119))),
        },
        second: Second(E::V(OtherSize120)),
    };
    const OTHER_SIZE120_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(120))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(120))),
        },
        second: Second(E::V(OtherSize121)),
    };
    const OTHER_SIZE121_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(121))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(121))),
        },
        second: Second(E::V(OtherSize122)),
    };
    const OTHER_SIZE122_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(122))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(122))),
        },
        second: Second(E::V(OtherSize123)),
    };
    const OTHER_SIZE123_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(123))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(123))),
        },
        second: Second(E::V(OtherSize124)),
    };
    const OTHER_SIZE124_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(124))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(124))),
        },
        second: Second(E::V(OtherSize125)),
    };
    const OTHER_SIZE125_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(125))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(125))),
        },
        second: Second(E::V(OtherSize126)),
    };
    const OTHER_SIZE126_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(126))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(126))),
        },
        second: Second(E::V(OtherSize127)),
    };
    const OTHER_SIZE127_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(127))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(127))),
        },
        second: Second(E::V(OtherSize128)),
    };
    const OTHER_SIZE128_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(128))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(128))),
        },
        second: Second(E::V(OtherSize129)),
    };
    const OTHER_SIZE129_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(129))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(129))),
        },
        second: Second(E::V(OtherSize130)),
    };
    const OTHER_SIZE130_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(130))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(130))),
        },
        second: Second(E::V(OtherSize131)),
    };
    const OTHER_SIZE131_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(131))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(131))),
        },
        second: Second(E::V(OtherSize132)),
    };
    const OTHER_SIZE132_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(132))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(132))),
        },
        second: Second(E::V(OtherSize133)),
    };
    const OTHER_SIZE133_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(133))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(133))),
        },
        second: Second(E::V(OtherSize134)),
    };
    const OTHER_SIZE134_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(134))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(134))),
        },
        second: Second(E::V(OtherSize135)),
    };
    const OTHER_SIZE135_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(135))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(135))),
        },
        second: Second(E::V(OtherSize136)),
    };
    const OTHER_SIZE136_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(136))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(136))),
        },
        second: Second(E::V(OtherSize137)),
    };
    const OTHER_SIZE137_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(137))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(137))),
        },
        second: Second(E::V(OtherSize138)),
    };
    const OTHER_SIZE138_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(138))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(138))),
        },
        second: Second(E::V(OtherSize139)),
    };
    const OTHER_SIZE139_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(139))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(139))),
        },
        second: Second(E::V(OtherSize140)),
    };
    const OTHER_SIZE140_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(140))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(140))),
        },
        second: Second(E::V(OtherSize141)),
    };
    const OTHER_SIZE141_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(141))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(141))),
        },
        second: Second(E::V(OtherSize142)),
    };
    const OTHER_SIZE142_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(142))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(142))),
        },
        second: Second(E::V(OtherSize143)),
    };
    const OTHER_SIZE143_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(143))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(143))),
        },
        second: Second(E::V(OtherSize144)),
    };
    const OTHER_SIZE144_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(144))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(144))),
        },
        second: Second(E::V(OtherSize145)),
    };
    const OTHER_SIZE145_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(145))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(145))),
        },
        second: Second(E::V(OtherSize146)),
    };
    const OTHER_SIZE146_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(146))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(146))),
        },
        second: Second(E::V(OtherSize147)),
    };
    const OTHER_SIZE147_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(147))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(147))),
        },
        second: Second(E::V(OtherSize148)),
    };
    const OTHER_SIZE148_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(148))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(148))),
        },
        second: Second(E::V(OtherSize149)),
    };
    const OTHER_SIZE149_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(149))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(149))),
        },
        second: Second(E::V(OtherSize150)),
    };
    const OTHER_SIZE150_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(150))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(150))),
        },
        second: Second(E::V(OtherSize151)),
    };
    const OTHER_SIZE151_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(151))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(151))),
        },
        second: Second(E::V(OtherSize152)),
    };
    const OTHER_SIZE152_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(152))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(152))),
        },
        second: Second(E::V(OtherSize153)),
    };
    const OTHER_SIZE153_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(153))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(153))),
        },
        second: Second(E::V(OtherSize154)),
    };
    const OTHER_SIZE154_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(154))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(154))),
        },
        second: Second(E::V(OtherSize155)),
    };
    const OTHER_SIZE155_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(155))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(155))),
        },
        second: Second(E::V(OtherSize156)),
    };
    const OTHER_SIZE156_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(156))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(156))),
        },
        second: Second(E::V(OtherSize157)),
    };
    const OTHER_SIZE157_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(157))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(157))),
        },
        second: Second(E::V(OtherSize158)),
    };
    const OTHER_SIZE158_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(158))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(158))),
        },
        second: Second(E::V(OtherSize159)),
    };
    const OTHER_SIZE159_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(159))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(159))),
        },
        second: Second(E::V(OtherSize160)),
    };
    const OTHER_SIZE160_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(160))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(160))),
        },
        second: Second(E::V(OtherSize161)),
    };
    const OTHER_SIZE161_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(161))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(161))),
        },
        second: Second(E::V(OtherSize162)),
    };
    const OTHER_SIZE162_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(162))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(162))),
        },
        second: Second(E::V(OtherSize163)),
    };
    const OTHER_SIZE163_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(163))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(163))),
        },
        second: Second(E::V(OtherSize164)),
    };
    const OTHER_SIZE164_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(164))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(164))),
        },
        second: Second(E::V(OtherSize165)),
    };
    const OTHER_SIZE165_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(165))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(165))),
        },
        second: Second(E::V(OtherSize166)),
    };
    const OTHER_SIZE166_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(166))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(166))),
        },
        second: Second(E::V(OtherSize167)),
    };
    const OTHER_SIZE167_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(167))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(167))),
        },
        second: Second(E::V(OtherSize168)),
    };
    const OTHER_SIZE168_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(168))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(168))),
        },
        second: Second(E::V(OtherSize169)),
    };
    const OTHER_SIZE169_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(169))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(169))),
        },
        second: Second(E::V(OtherSize170)),
    };
    const OTHER_SIZE170_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(170))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(170))),
        },
        second: Second(E::V(OtherSize171)),
    };
    const OTHER_SIZE171_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(171))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(171))),
        },
        second: Second(E::V(OtherSize172)),
    };
    const OTHER_SIZE172_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(172))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(172))),
        },
        second: Second(E::V(OtherSize173)),
    };
    const OTHER_SIZE173_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(173))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(173))),
        },
        second: Second(E::V(OtherSize174)),
    };
    const OTHER_SIZE174_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(174))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(174))),
        },
        second: Second(E::V(OtherSize175)),
    };
    const OTHER_SIZE175_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(175))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(175))),
        },
        second: Second(E::V(OtherSize176)),
    };
    const OTHER_SIZE176_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(176))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(176))),
        },
        second: Second(E::V(OtherSize177)),
    };
    const OTHER_SIZE177_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(177))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(177))),
        },
        second: Second(E::V(OtherSize178)),
    };
    const OTHER_SIZE178_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(178))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(178))),
        },
        second: Second(E::V(OtherSize179)),
    };
    const OTHER_SIZE179_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(179))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(179))),
        },
        second: Second(E::V(OtherSize180)),
    };
    const OTHER_SIZE180_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(180))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(180))),
        },
        second: Second(E::V(OtherSize181)),
    };
    const OTHER_SIZE181_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(181))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(181))),
        },
        second: Second(E::V(OtherSize182)),
    };
    const OTHER_SIZE182_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(182))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(182))),
        },
        second: Second(E::V(OtherSize183)),
    };
    const OTHER_SIZE183_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(183))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(183))),
        },
        second: Second(E::V(OtherSize184)),
    };
    const OTHER_SIZE184_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(184))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(184))),
        },
        second: Second(E::V(OtherSize185)),
    };
    const OTHER_SIZE185_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(185))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(185))),
        },
        second: Second(E::V(OtherSize186)),
    };
    const OTHER_SIZE186_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(186))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(186))),
        },
        second: Second(E::V(OtherSize187)),
    };
    const OTHER_SIZE187_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(187))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(187))),
        },
        second: Second(E::V(OtherSize188)),
    };
    const OTHER_SIZE188_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(188))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(188))),
        },
        second: Second(E::V(OtherSize189)),
    };
    const OTHER_SIZE189_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(189))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(189))),
        },
        second: Second(E::V(OtherSize190)),
    };
    const OTHER_SIZE190_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(190))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(190))),
        },
        second: Second(E::V(OtherSize191)),
    };
    const OTHER_SIZE191_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(191))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(191))),
        },
        second: Second(E::V(OtherSize192)),
    };
    const OTHER_SIZE192_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(192))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(192))),
        },
        second: Second(E::V(OtherSize193)),
    };
    const OTHER_SIZE193_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(193))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(193))),
        },
        second: Second(E::V(OtherSize194)),
    };
    const OTHER_SIZE194_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(194))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(194))),
        },
        second: Second(E::V(OtherSize195)),
    };
    const OTHER_SIZE195_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(195))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(195))),
        },
        second: Second(E::V(OtherSize196)),
    };
    const OTHER_SIZE196_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(196))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(196))),
        },
        second: Second(E::V(OtherSize197)),
    };
    const OTHER_SIZE197_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(197))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(197))),
        },
        second: Second(E::V(OtherSize198)),
    };
    const OTHER_SIZE198_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(198))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(198))),
        },
        second: Second(E::V(OtherSize199)),
    };
    const OTHER_SIZE199_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(199))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(199))),
        },
        second: Second(E::V(OtherSize200)),
    };
    const OTHER_SIZE200_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(200))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(200))),
        },
        second: Second(E::V(OtherSize201)),
    };
    const OTHER_SIZE201_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(201))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(201))),
        },
        second: Second(E::V(OtherSize202)),
    };
    const OTHER_SIZE202_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(202))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(202))),
        },
        second: Second(E::V(OtherSize203)),
    };
    const OTHER_SIZE203_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(203))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(203))),
        },
        second: Second(E::V(OtherSize204)),
    };
    const OTHER_SIZE204_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(204))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(204))),
        },
        second: Second(E::V(OtherSize205)),
    };
    const OTHER_SIZE205_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(205))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(205))),
        },
        second: Second(E::V(OtherSize206)),
    };
    const OTHER_SIZE206_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(206))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(206))),
        },
        second: Second(E::V(OtherSize207)),
    };
    const OTHER_SIZE207_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(207))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(207))),
        },
        second: Second(E::V(OtherSize208)),
    };
    const OTHER_SIZE208_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(208))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(208))),
        },
        second: Second(E::V(OtherSize209)),
    };
    const OTHER_SIZE209_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(209))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(209))),
        },
        second: Second(E::V(OtherSize210)),
    };
    const OTHER_SIZE210_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(210))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(210))),
        },
        second: Second(E::V(OtherSize211)),
    };
    const OTHER_SIZE211_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(211))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(211))),
        },
        second: Second(E::V(OtherSize212)),
    };
    const OTHER_SIZE212_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(212))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(212))),
        },
        second: Second(E::V(OtherSize213)),
    };
    const OTHER_SIZE213_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(213))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(213))),
        },
        second: Second(E::V(OtherSize214)),
    };
    const OTHER_SIZE214_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(214))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(214))),
        },
        second: Second(E::V(OtherSize215)),
    };
    const OTHER_SIZE215_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(215))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(215))),
        },
        second: Second(E::V(OtherSize216)),
    };
    const OTHER_SIZE216_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(216))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(216))),
        },
        second: Second(E::V(OtherSize217)),
    };
    const OTHER_SIZE217_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(217))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(217))),
        },
        second: Second(E::V(OtherSize218)),
    };
    const OTHER_SIZE218_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(218))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(218))),
        },
        second: Second(E::V(OtherSize219)),
    };
    const OTHER_SIZE219_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(219))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(219))),
        },
        second: Second(E::V(OtherSize220)),
    };
    const OTHER_SIZE220_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(220))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(220))),
        },
        second: Second(E::V(OtherSize221)),
    };
    const OTHER_SIZE221_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(221))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(221))),
        },
        second: Second(E::V(OtherSize222)),
    };
    const OTHER_SIZE222_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(222))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(222))),
        },
        second: Second(E::V(OtherSize223)),
    };
    const OTHER_SIZE223_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(223))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(223))),
        },
        second: Second(E::V(OtherSize224)),
    };
    const OTHER_SIZE224_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(224))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(224))),
        },
        second: Second(E::V(OtherSize225)),
    };
    const OTHER_SIZE225_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(225))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(225))),
        },
        second: Second(E::V(OtherSize226)),
    };
    const OTHER_SIZE226_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(226))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(226))),
        },
        second: Second(E::V(OtherSize227)),
    };
    const OTHER_SIZE227_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(227))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(227))),
        },
        second: Second(E::V(OtherSize228)),
    };
    const OTHER_SIZE228_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(228))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(228))),
        },
        second: Second(E::V(OtherSize229)),
    };
    const OTHER_SIZE229_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(229))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(229))),
        },
        second: Second(E::V(OtherSize230)),
    };
    const OTHER_SIZE230_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(230))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(230))),
        },
        second: Second(E::V(OtherSize231)),
    };
    const OTHER_SIZE231_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(231))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(231))),
        },
        second: Second(E::V(OtherSize232)),
    };
    const OTHER_SIZE232_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(232))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(232))),
        },
        second: Second(E::V(OtherSize233)),
    };
    const OTHER_SIZE233_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(233))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(233))),
        },
        second: Second(E::V(OtherSize234)),
    };
    const OTHER_SIZE234_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(234))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(234))),
        },
        second: Second(E::V(OtherSize235)),
    };
    const OTHER_SIZE235_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(235))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(235))),
        },
        second: Second(E::V(OtherSize236)),
    };
    const OTHER_SIZE236_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(236))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(236))),
        },
        second: Second(E::V(OtherSize237)),
    };
    const OTHER_SIZE237_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(237))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(237))),
        },
        second: Second(E::V(OtherSize238)),
    };
    const OTHER_SIZE238_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(238))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(238))),
        },
        second: Second(E::V(OtherSize239)),
    };
    const OTHER_SIZE239_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(239))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(239))),
        },
        second: Second(E::V(OtherSize240)),
    };
    const OTHER_SIZE240_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(240))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(240))),
        },
        second: Second(E::V(OtherSize241)),
    };
    const OTHER_SIZE241_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(241))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(241))),
        },
        second: Second(E::V(OtherSize242)),
    };
    const OTHER_SIZE242_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(242))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(242))),
        },
        second: Second(E::V(OtherSize243)),
    };
    const OTHER_SIZE243_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(243))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(243))),
        },
        second: Second(E::V(OtherSize244)),
    };
    const OTHER_SIZE244_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(244))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(244))),
        },
        second: Second(E::V(OtherSize245)),
    };
    const OTHER_SIZE245_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(245))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(245))),
        },
        second: Second(E::V(OtherSize246)),
    };
    const OTHER_SIZE246_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(246))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(246))),
        },
        second: Second(E::V(OtherSize247)),
    };
    const OTHER_SIZE247_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(247))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(247))),
        },
        second: Second(E::V(OtherSize248)),
    };
    const OTHER_SIZE248_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(248))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(248))),
        },
        second: Second(E::V(OtherSize249)),
    };
    const OTHER_SIZE249_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(249))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(249))),
        },
        second: Second(E::V(OtherSize250)),
    };
    const OTHER_SIZE250_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(250))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(250))),
        },
        second: Second(E::V(OtherSize251)),
    };
    const OTHER_SIZE251_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(251))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(251))),
        },
        second: Second(E::V(OtherSize252)),
    };
    const OTHER_SIZE252_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(252))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(252))),
        },
        second: Second(E::V(OtherSize253)),
    };
    const OTHER_SIZE253_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(253))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(253))),
        },
        second: Second(E::V(OtherSize254)),
    };
    const OTHER_SIZE254_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(254))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(254))),
        },
        second: Second(E::V(OtherSize255)),
    };
    const OTHER_SIZE255_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(255))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(255))),
        },
        second: Second(E::V(OtherSize256)),
    };
    const OTHER_SIZE256_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(256))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(256))),
        },
        second: Second(E::V(OtherSize257)),
    };
    const OTHER_SIZE257_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(257))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(257))),
        },
        second: Second(E::V(OtherSize258)),
    };
    const OTHER_SIZE258_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(258))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(258))),
        },
        second: Second(E::V(OtherSize259)),
    };
    const OTHER_SIZE259_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(259))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(259))),
        },
        second: Second(E::V(OtherSize260)),
    };
    const OTHER_SIZE260_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(260))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(260))),
        },
        second: Second(E::V(OtherSize261)),
    };
    const OTHER_SIZE261_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(261))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(261))),
        },
        second: Second(E::V(OtherSize262)),
    };
    const OTHER_SIZE262_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(262))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(262))),
        },
        second: Second(E::V(OtherSize263)),
    };
    const OTHER_SIZE263_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(263))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(263))),
        },
        second: Second(E::V(OtherSize264)),
    };
    const OTHER_SIZE264_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(264))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(264))),
        },
        second: Second(E::V(OtherSize265)),
    };
    const OTHER_SIZE265_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(265))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(265))),
        },
        second: Second(E::V(OtherSize266)),
    };
    const OTHER_SIZE266_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(266))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(266))),
        },
        second: Second(E::V(OtherSize267)),
    };
    const OTHER_SIZE267_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(267))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(267))),
        },
        second: Second(E::V(OtherSize268)),
    };
    const OTHER_SIZE268_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(268))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(268))),
        },
        second: Second(E::V(OtherSize269)),
    };
    const OTHER_SIZE269_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(269))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(269))),
        },
        second: Second(E::V(OtherSize270)),
    };
    const OTHER_SIZE270_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(270))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(270))),
        },
        second: Second(E::V(OtherSize271)),
    };
    const OTHER_SIZE271_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(271))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(271))),
        },
        second: Second(E::V(OtherSize272)),
    };
    const OTHER_SIZE272_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(272))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(272))),
        },
        second: Second(E::V(OtherSize273)),
    };
    const OTHER_SIZE273_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(273))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(273))),
        },
        second: Second(E::V(OtherSize274)),
    };
    const OTHER_SIZE274_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(274))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(274))),
        },
        second: Second(E::V(OtherSize275)),
    };
    const OTHER_SIZE275_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(275))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(275))),
        },
        second: Second(E::V(OtherSize276)),
    };
    const OTHER_SIZE276_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(276))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(276))),
        },
        second: Second(E::V(OtherSize277)),
    };
    const OTHER_SIZE277_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(277))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(277))),
        },
        second: Second(E::V(OtherSize278)),
    };
    const OTHER_SIZE278_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(278))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(278))),
        },
        second: Second(E::V(OtherSize279)),
    };
    const OTHER_SIZE279_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(279))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(279))),
        },
        second: Second(E::V(OtherSize280)),
    };
    const OTHER_SIZE280_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(280))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(280))),
        },
        second: Second(E::V(OtherSize281)),
    };
    const OTHER_SIZE281_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(281))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(281))),
        },
        second: Second(E::V(OtherSize282)),
    };
    const OTHER_SIZE282_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(282))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(282))),
        },
        second: Second(E::V(OtherSize283)),
    };
    const OTHER_SIZE283_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(283))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(283))),
        },
        second: Second(E::V(OtherSize284)),
    };
    const OTHER_SIZE284_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(284))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(284))),
        },
        second: Second(E::V(OtherSize285)),
    };
    const OTHER_SIZE285_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(285))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(285))),
        },
        second: Second(E::V(OtherSize286)),
    };
    const OTHER_SIZE286_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(286))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(286))),
        },
        second: Second(E::V(OtherSize287)),
    };
    const OTHER_SIZE287_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(287))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(287))),
        },
        second: Second(E::V(OtherSize288)),
    };
    const OTHER_SIZE288_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(288))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(288))),
        },
        second: Second(E::V(OtherSize289)),
    };
    const OTHER_SIZE289_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(289))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(289))),
        },
        second: Second(E::V(OtherSize290)),
    };
    const OTHER_SIZE290_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(290))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(290))),
        },
        second: Second(E::V(OtherSize291)),
    };
    const OTHER_SIZE291_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(291))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(291))),
        },
        second: Second(E::V(OtherSize292)),
    };
    const OTHER_SIZE292_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(292))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(292))),
        },
        second: Second(E::V(OtherSize293)),
    };
    const OTHER_SIZE293_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(293))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(293))),
        },
        second: Second(E::V(OtherSize294)),
    };
    const OTHER_SIZE294_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(294))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(294))),
        },
        second: Second(E::V(OtherSize295)),
    };
    const OTHER_SIZE295_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(295))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(295))),
        },
        second: Second(E::V(OtherSize296)),
    };
    const OTHER_SIZE296_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(296))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(296))),
        },
        second: Second(E::V(OtherSize297)),
    };
    const OTHER_SIZE297_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(297))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(297))),
        },
        second: Second(E::V(OtherSize298)),
    };
    const OTHER_SIZE298_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(298))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(298))),
        },
        second: Second(E::V(OtherSize299)),
    };
    const OTHER_SIZE299_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(299))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(299))),
        },
        second: Second(E::V(OtherSize300)),
    };
    const OTHER_SIZE300_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(300))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(300))),
        },
        second: Second(E::V(OtherSize301)),
    };
    const OTHER_SIZE301_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(301))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(301))),
        },
        second: Second(E::V(OtherSize302)),
    };
    const OTHER_SIZE302_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(302))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(302))),
        },
        second: Second(E::V(OtherSize303)),
    };
    const OTHER_SIZE303_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(303))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(303))),
        },
        second: Second(E::V(OtherSize304)),
    };
    const OTHER_SIZE304_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(304))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(304))),
        },
        second: Second(E::V(OtherSize305)),
    };
    const OTHER_SIZE305_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(305))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(305))),
        },
        second: Second(E::V(OtherSize306)),
    };
    const OTHER_SIZE306_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(306))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(306))),
        },
        second: Second(E::V(OtherSize307)),
    };
    const OTHER_SIZE307_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(307))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(307))),
        },
        second: Second(E::V(OtherSize308)),
    };
    const OTHER_SIZE308_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(308))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(308))),
        },
        second: Second(E::V(OtherSize309)),
    };
    const OTHER_SIZE309_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(309))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(309))),
        },
        second: Second(E::V(OtherSize310)),
    };
    const OTHER_SIZE310_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(310))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(310))),
        },
        second: Second(E::V(OtherSize311)),
    };
    const OTHER_SIZE311_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(311))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(311))),
        },
        second: Second(E::V(OtherSize312)),
    };
    const OTHER_SIZE312_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(312))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(312))),
        },
        second: Second(E::V(OtherSize313)),
    };
    const OTHER_SIZE313_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(313))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(313))),
        },
        second: Second(E::V(OtherSize314)),
    };
    const OTHER_SIZE314_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(314))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(314))),
        },
        second: Second(E::V(OtherSize315)),
    };
    const OTHER_SIZE315_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(315))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(315))),
        },
        second: Second(E::V(OtherSize316)),
    };
    const OTHER_SIZE316_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(316))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(316))),
        },
        second: Second(E::V(OtherSize317)),
    };
    const OTHER_SIZE317_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(317))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(317))),
        },
        second: Second(E::V(OtherSize318)),
    };
    const OTHER_SIZE318_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(318))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(318))),
        },
        second: Second(E::V(OtherSize319)),
    };
    const OTHER_SIZE319_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(319))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(319))),
        },
        second: Second(E::V(OtherSize320)),
    };
    const OTHER_SIZE320_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(320))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(320))),
        },
        second: Second(E::V(OtherSize321)),
    };
    const OTHER_SIZE321_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(321))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(321))),
        },
        second: Second(E::V(OtherSize322)),
    };
    const OTHER_SIZE322_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(322))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(322))),
        },
        second: Second(E::V(OtherSize323)),
    };
    const OTHER_SIZE323_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(323))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(323))),
        },
        second: Second(E::V(OtherSize324)),
    };
    const OTHER_SIZE324_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(324))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(324))),
        },
        second: Second(E::V(OtherSize325)),
    };
    const OTHER_SIZE325_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(325))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(325))),
        },
        second: Second(E::V(OtherSize326)),
    };
    const OTHER_SIZE326_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(326))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(326))),
        },
        second: Second(E::V(OtherSize327)),
    };
    const OTHER_SIZE327_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(327))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(327))),
        },
        second: Second(E::V(OtherSize328)),
    };
    const OTHER_SIZE328_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(328))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(328))),
        },
        second: Second(E::V(OtherSize329)),
    };
    const OTHER_SIZE329_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(329))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(329))),
        },
        second: Second(E::V(OtherSize330)),
    };
    const OTHER_SIZE330_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(330))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(330))),
        },
        second: Second(E::V(OtherSize331)),
    };
    const OTHER_SIZE331_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(331))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(331))),
        },
        second: Second(E::V(OtherSize332)),
    };
    const OTHER_SIZE332_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(332))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(332))),
        },
        second: Second(E::V(OtherSize333)),
    };
    const OTHER_SIZE333_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(333))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(333))),
        },
        second: Second(E::V(OtherSize334)),
    };
    const OTHER_SIZE334_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(334))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(334))),
        },
        second: Second(E::V(OtherSize335)),
    };
    const OTHER_SIZE335_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(335))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(335))),
        },
        second: Second(E::V(OtherSize336)),
    };
    const OTHER_SIZE336_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(336))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(336))),
        },
        second: Second(E::V(OtherSize337)),
    };
    const OTHER_SIZE337_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(337))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(337))),
        },
        second: Second(E::V(OtherSize338)),
    };
    const OTHER_SIZE338_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(338))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(338))),
        },
        second: Second(E::V(OtherSize339)),
    };
    const OTHER_SIZE339_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(339))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(339))),
        },
        second: Second(E::V(OtherSize340)),
    };
    const OTHER_SIZE340_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(340))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(340))),
        },
        second: Second(E::V(OtherSize341)),
    };
    const OTHER_SIZE341_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(341))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(341))),
        },
        second: Second(E::V(OtherSize342)),
    };
    const OTHER_SIZE342_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(342))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(342))),
        },
        second: Second(E::V(OtherSize343)),
    };
    const OTHER_SIZE343_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(343))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(343))),
        },
        second: Second(E::V(OtherSize344)),
    };
    const OTHER_SIZE344_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(344))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(344))),
        },
        second: Second(E::V(OtherSize345)),
    };
    const OTHER_SIZE345_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(345))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(345))),
        },
        second: Second(E::V(OtherSize346)),
    };
    const OTHER_SIZE346_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(346))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(346))),
        },
        second: Second(E::V(OtherSize347)),
    };
    const OTHER_SIZE347_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(347))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(347))),
        },
        second: Second(E::V(OtherSize348)),
    };
    const OTHER_SIZE348_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(348))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(348))),
        },
        second: Second(E::V(OtherSize349)),
    };
    const OTHER_SIZE349_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(349))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(349))),
        },
        second: Second(E::V(OtherSize350)),
    };
    const OTHER_SIZE350_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(350))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(350))),
        },
        second: Second(E::V(OtherSize351)),
    };
    const OTHER_SIZE351_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(351))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(351))),
        },
        second: Second(E::V(OtherSize352)),
    };
    const OTHER_SIZE352_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(352))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(352))),
        },
        second: Second(E::V(OtherSize353)),
    };
    const OTHER_SIZE353_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(353))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(353))),
        },
        second: Second(E::V(OtherSize354)),
    };
    const OTHER_SIZE354_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(354))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(354))),
        },
        second: Second(E::V(OtherSize355)),
    };
    const OTHER_SIZE355_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(355))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(355))),
        },
        second: Second(E::V(OtherSize356)),
    };
    const OTHER_SIZE356_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(356))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(356))),
        },
        second: Second(E::V(OtherSize357)),
    };
    const OTHER_SIZE357_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(357))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(357))),
        },
        second: Second(E::V(OtherSize358)),
    };
    const OTHER_SIZE358_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(358))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(358))),
        },
        second: Second(E::V(OtherSize359)),
    };
    const OTHER_SIZE359_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(359))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(359))),
        },
        second: Second(E::V(OtherSize360)),
    };
    const OTHER_SIZE360_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(360))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(360))),
        },
        second: Second(E::V(OtherSize361)),
    };
    const OTHER_SIZE361_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(361))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(361))),
        },
        second: Second(E::V(OtherSize362)),
    };
    const OTHER_SIZE362_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(362))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(362))),
        },
        second: Second(E::V(OtherSize363)),
    };
    const OTHER_SIZE363_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(363))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(363))),
        },
        second: Second(E::V(OtherSize364)),
    };
    const OTHER_SIZE364_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(364))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(364))),
        },
        second: Second(E::V(OtherSize365)),
    };
    const OTHER_SIZE365_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(365))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(365))),
        },
        second: Second(E::V(OtherSize366)),
    };
    const OTHER_SIZE366_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(366))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(366))),
        },
        second: Second(E::V(OtherSize367)),
    };
    const OTHER_SIZE367_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(367))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(367))),
        },
        second: Second(E::V(OtherSize368)),
    };
    const OTHER_SIZE368_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(368))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(368))),
        },
        second: Second(E::V(OtherSize369)),
    };
    const OTHER_SIZE369_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(369))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(369))),
        },
        second: Second(E::V(OtherSize370)),
    };
    const OTHER_SIZE370_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(370))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(370))),
        },
        second: Second(E::V(OtherSize371)),
    };
    const OTHER_SIZE371_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(371))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(371))),
        },
        second: Second(E::V(OtherSize372)),
    };
    const OTHER_SIZE372_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(372))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(372))),
        },
        second: Second(E::V(OtherSize373)),
    };
    const OTHER_SIZE373_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(373))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(373))),
        },
        second: Second(E::V(OtherSize374)),
    };
    const OTHER_SIZE374_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(374))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(374))),
        },
        second: Second(E::V(OtherSize375)),
    };
    const OTHER_SIZE375_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(375))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(375))),
        },
        second: Second(E::V(OtherSize376)),
    };
    const OTHER_SIZE376_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(376))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(376))),
        },
        second: Second(E::V(OtherSize377)),
    };
    const OTHER_SIZE377_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(377))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(377))),
        },
        second: Second(E::V(OtherSize378)),
    };
    const OTHER_SIZE378_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(378))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(378))),
        },
        second: Second(E::V(OtherSize379)),
    };
    const OTHER_SIZE379_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(379))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(379))),
        },
        second: Second(E::V(OtherSize380)),
    };
    const OTHER_SIZE380_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(380))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(380))),
        },
        second: Second(E::V(OtherSize381)),
    };
    const OTHER_SIZE381_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(381))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(381))),
        },
        second: Second(E::V(OtherSize382)),
    };
    const OTHER_SIZE382_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(382))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(382))),
        },
        second: Second(E::V(OtherSize383)),
    };
    const OTHER_SIZE383_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(383))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(383))),
        },
        second: Second(E::V(OtherSize384)),
    };
    const OTHER_SIZE384_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(384))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(384))),
        },
        second: Second(E::V(OtherSize385)),
    };
    const OTHER_SIZE385_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(385))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(385))),
        },
        second: Second(E::V(OtherSize386)),
    };
    const OTHER_SIZE386_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(386))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(386))),
        },
        second: Second(E::V(OtherSize387)),
    };
    const OTHER_SIZE387_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(387))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(387))),
        },
        second: Second(E::V(OtherSize388)),
    };
    const OTHER_SIZE388_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(388))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(388))),
        },
        second: Second(E::V(OtherSize389)),
    };
    const OTHER_SIZE389_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(389))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(389))),
        },
        second: Second(E::V(OtherSize390)),
    };
    const OTHER_SIZE390_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(390))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(390))),
        },
        second: Second(E::V(OtherSize391)),
    };
    const OTHER_SIZE391_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(391))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(391))),
        },
        second: Second(E::V(OtherSize392)),
    };
    const OTHER_SIZE392_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(392))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(392))),
        },
        second: Second(E::V(OtherSize393)),
    };
    const OTHER_SIZE393_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(393))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(393))),
        },
        second: Second(E::V(OtherSize394)),
    };
    const OTHER_SIZE394_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(394))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(394))),
        },
        second: Second(E::V(OtherSize395)),
    };
    const OTHER_SIZE395_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(395))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(395))),
        },
        second: Second(E::V(OtherSize396)),
    };
    const OTHER_SIZE396_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(396))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(396))),
        },
        second: Second(E::V(OtherSize397)),
    };
    const OTHER_SIZE397_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(397))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(397))),
        },
        second: Second(E::V(OtherSize398)),
    };
    const OTHER_SIZE398_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(398))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(398))),
        },
        second: Second(E::V(OtherSize399)),
    };
    const OTHER_SIZE399_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(399))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(399))),
        },
        second: Second(E::V(OtherSize400)),
    };
    const OTHER_SIZE400_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(400))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(400))),
        },
        second: Second(E::V(OtherSize401)),
    };
    const OTHER_SIZE401_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(401))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(401))),
        },
        second: Second(E::V(OtherSize402)),
    };
    const OTHER_SIZE402_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(402))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(402))),
        },
        second: Second(E::V(OtherSize403)),
    };
    const OTHER_SIZE403_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(403))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(403))),
        },
        second: Second(E::V(OtherSize404)),
    };
    const OTHER_SIZE404_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(404))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(404))),
        },
        second: Second(E::V(OtherSize405)),
    };
    const OTHER_SIZE405_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(405))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(405))),
        },
        second: Second(E::V(OtherSize406)),
    };
    const OTHER_SIZE406_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(406))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(406))),
        },
        second: Second(E::V(OtherSize407)),
    };
    const OTHER_SIZE407_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(407))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(407))),
        },
        second: Second(E::V(OtherSize408)),
    };
    const OTHER_SIZE408_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(408))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(408))),
        },
        second: Second(E::V(OtherSize409)),
    };
    const OTHER_SIZE409_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(409))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(409))),
        },
        second: Second(E::V(OtherSize410)),
    };
    const OTHER_SIZE410_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(410))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(410))),
        },
        second: Second(E::V(OtherSize411)),
    };
    const OTHER_SIZE411_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(411))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(411))),
        },
        second: Second(E::V(OtherSize412)),
    };
    const OTHER_SIZE412_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(412))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(412))),
        },
        second: Second(E::V(OtherSize413)),
    };
    const OTHER_SIZE413_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(413))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(413))),
        },
        second: Second(E::V(OtherSize414)),
    };
    const OTHER_SIZE414_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(414))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(414))),
        },
        second: Second(E::V(OtherSize415)),
    };
    const OTHER_SIZE415_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(415))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(415))),
        },
        second: Second(E::V(OtherSize416)),
    };
    const OTHER_SIZE416_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(416))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(416))),
        },
        second: Second(E::V(OtherSize417)),
    };
    const OTHER_SIZE417_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(417))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(417))),
        },
        second: Second(E::V(OtherSize418)),
    };
    const OTHER_SIZE418_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(418))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(418))),
        },
        second: Second(E::V(OtherSize419)),
    };
    const OTHER_SIZE419_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(419))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(419))),
        },
        second: Second(E::V(OtherSize420)),
    };
    const OTHER_SIZE420_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(420))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(420))),
        },
        second: Second(E::V(OtherSize421)),
    };
    const OTHER_SIZE421_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(421))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(421))),
        },
        second: Second(E::V(OtherSize422)),
    };
    const OTHER_SIZE422_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(422))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(422))),
        },
        second: Second(E::V(OtherSize423)),
    };
    const OTHER_SIZE423_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(423))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(423))),
        },
        second: Second(E::V(OtherSize424)),
    };
    const OTHER_SIZE424_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(424))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(424))),
        },
        second: Second(E::V(OtherSize425)),
    };
    const OTHER_SIZE425_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(425))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(425))),
        },
        second: Second(E::V(OtherSize426)),
    };
    const OTHER_SIZE426_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(426))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(426))),
        },
        second: Second(E::V(OtherSize427)),
    };
    const OTHER_SIZE427_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(427))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(427))),
        },
        second: Second(E::V(OtherSize428)),
    };
    const OTHER_SIZE428_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(428))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(428))),
        },
        second: Second(E::V(OtherSize429)),
    };
    const OTHER_SIZE429_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(429))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(429))),
        },
        second: Second(E::V(OtherSize430)),
    };
    const OTHER_SIZE430_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(430))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(430))),
        },
        second: Second(E::V(OtherSize431)),
    };
    const OTHER_SIZE431_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(431))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(431))),
        },
        second: Second(E::V(OtherSize432)),
    };
    const OTHER_SIZE432_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(432))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(432))),
        },
        second: Second(E::V(OtherSize433)),
    };
    const OTHER_SIZE433_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(433))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(433))),
        },
        second: Second(E::V(OtherSize434)),
    };
    const OTHER_SIZE434_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(434))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(434))),
        },
        second: Second(E::V(OtherSize435)),
    };
    const OTHER_SIZE435_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(435))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(435))),
        },
        second: Second(E::V(OtherSize436)),
    };
    const OTHER_SIZE436_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(436))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(436))),
        },
        second: Second(E::V(OtherSize437)),
    };
    const OTHER_SIZE437_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(437))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(437))),
        },
        second: Second(E::V(OtherSize438)),
    };
    const OTHER_SIZE438_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(438))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(438))),
        },
        second: Second(E::V(OtherSize439)),
    };
    const OTHER_SIZE439_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(439))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(439))),
        },
        second: Second(E::V(OtherSize440)),
    };
    const OTHER_SIZE440_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(440))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(440))),
        },
        second: Second(E::V(OtherSize441)),
    };
    const OTHER_SIZE441_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(441))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(441))),
        },
        second: Second(E::V(OtherSize442)),
    };
    const OTHER_SIZE442_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(442))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(442))),
        },
        second: Second(E::V(OtherSize443)),
    };
    const OTHER_SIZE443_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(443))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(443))),
        },
        second: Second(E::V(OtherSize444)),
    };
    const OTHER_SIZE444_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(444))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(444))),
        },
        second: Second(E::V(OtherSize445)),
    };
    const OTHER_SIZE445_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(445))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(445))),
        },
        second: Second(E::V(OtherSize446)),
    };
    const OTHER_SIZE446_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(446))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(446))),
        },
        second: Second(E::V(OtherSize447)),
    };
    const OTHER_SIZE447_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(447))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(447))),
        },
        second: Second(E::V(OtherSize448)),
    };
    const OTHER_SIZE448_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(448))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(448))),
        },
        second: Second(E::V(OtherSize449)),
    };
    const OTHER_SIZE449_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(449))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(449))),
        },
        second: Second(E::V(OtherSize450)),
    };
    const OTHER_SIZE450_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(450))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(450))),
        },
        second: Second(E::V(OtherSize451)),
    };
    const OTHER_SIZE451_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(451))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(451))),
        },
        second: Second(E::V(OtherSize452)),
    };
    const OTHER_SIZE452_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(452))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(452))),
        },
        second: Second(E::V(OtherSize453)),
    };
    const OTHER_SIZE453_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(453))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(453))),
        },
        second: Second(E::V(OtherSize454)),
    };
    const OTHER_SIZE454_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(454))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(454))),
        },
        second: Second(E::V(OtherSize455)),
    };
    const OTHER_SIZE455_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(455))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(455))),
        },
        second: Second(E::V(OtherSize456)),
    };
    const OTHER_SIZE456_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(456))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(456))),
        },
        second: Second(E::V(OtherSize457)),
    };
    const OTHER_SIZE457_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(457))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(457))),
        },
        second: Second(E::V(OtherSize458)),
    };
    const OTHER_SIZE458_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(458))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(458))),
        },
        second: Second(E::V(OtherSize459)),
    };
    const OTHER_SIZE459_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(459))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(459))),
        },
        second: Second(E::V(OtherSize460)),
    };
    const OTHER_SIZE460_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(460))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(460))),
        },
        second: Second(E::V(OtherSize461)),
    };
    const OTHER_SIZE461_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(461))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(461))),
        },
        second: Second(E::V(OtherSize462)),
    };
    const OTHER_SIZE462_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(462))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(462))),
        },
        second: Second(E::V(OtherSize463)),
    };
    const OTHER_SIZE463_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(463))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(463))),
        },
        second: Second(E::V(OtherSize464)),
    };
    const OTHER_SIZE464_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(464))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(464))),
        },
        second: Second(E::V(OtherSize465)),
    };
    const OTHER_SIZE465_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(465))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(465))),
        },
        second: Second(E::V(OtherSize466)),
    };
    const OTHER_SIZE466_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(466))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(466))),
        },
        second: Second(E::V(OtherSize467)),
    };
    const OTHER_SIZE467_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(467))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(467))),
        },
        second: Second(E::V(OtherSize468)),
    };
    const OTHER_SIZE468_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(468))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(468))),
        },
        second: Second(E::V(OtherSize469)),
    };
    const OTHER_SIZE469_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(469))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(469))),
        },
        second: Second(E::V(OtherSize470)),
    };
    const OTHER_SIZE470_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(470))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(470))),
        },
        second: Second(E::V(OtherSize471)),
    };
    const OTHER_SIZE471_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(471))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(471))),
        },
        second: Second(E::V(OtherSize472)),
    };
    const OTHER_SIZE472_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(472))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(472))),
        },
        second: Second(E::V(OtherSize473)),
    };
    const OTHER_SIZE473_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(473))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(473))),
        },
        second: Second(E::V(OtherSize474)),
    };
    const OTHER_SIZE474_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(474))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(474))),
        },
        second: Second(E::V(OtherSize475)),
    };
    const OTHER_SIZE475_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(475))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(475))),
        },
        second: Second(E::V(OtherSize476)),
    };
    const OTHER_SIZE476_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(476))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(476))),
        },
        second: Second(E::V(OtherSize477)),
    };
    const OTHER_SIZE477_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(477))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(477))),
        },
        second: Second(E::V(OtherSize478)),
    };
    const OTHER_SIZE478_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(478))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(478))),
        },
        second: Second(E::V(OtherSize479)),
    };
    const OTHER_SIZE479_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(479))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(479))),
        },
        second: Second(E::V(OtherSize480)),
    };
    const OTHER_SIZE480_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(480))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(480))),
        },
        second: Second(E::V(OtherSize481)),
    };
    const OTHER_SIZE481_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(481))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(481))),
        },
        second: Second(E::V(OtherSize482)),
    };
    const OTHER_SIZE482_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(482))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(482))),
        },
        second: Second(E::V(OtherSize483)),
    };
    const OTHER_SIZE483_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(483))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(483))),
        },
        second: Second(E::V(OtherSize484)),
    };
    const OTHER_SIZE484_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(484))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(484))),
        },
        second: Second(E::V(OtherSize485)),
    };
    const OTHER_SIZE485_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(485))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(485))),
        },
        second: Second(E::V(OtherSize486)),
    };
    const OTHER_SIZE486_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(486))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(486))),
        },
        second: Second(E::V(OtherSize487)),
    };
    const OTHER_SIZE487_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(487))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(487))),
        },
        second: Second(E::V(OtherSize488)),
    };
    const OTHER_SIZE488_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(488))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(488))),
        },
        second: Second(E::V(OtherSize489)),
    };
    const OTHER_SIZE489_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(489))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(489))),
        },
        second: Second(E::V(OtherSize490)),
    };
    const OTHER_SIZE490_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(490))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(490))),
        },
        second: Second(E::V(OtherSize491)),
    };
    const OTHER_SIZE491_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(491))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(491))),
        },
        second: Second(E::V(OtherSize492)),
    };
    const OTHER_SIZE492_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(492))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(492))),
        },
        second: Second(E::V(OtherSize493)),
    };
    const OTHER_SIZE493_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(493))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(493))),
        },
        second: Second(E::V(OtherSize494)),
    };
    const OTHER_SIZE494_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(494))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(494))),
        },
        second: Second(E::V(OtherSize495)),
    };
    const OTHER_SIZE495_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(495))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(495))),
        },
        second: Second(E::V(OtherSize496)),
    };
    const OTHER_SIZE496_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(496))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(496))),
        },
        second: Second(E::V(OtherSize497)),
    };
    const OTHER_SIZE497_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(497))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(497))),
        },
        second: Second(E::V(OtherSize498)),
    };
    const OTHER_SIZE498_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(498))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(498))),
        },
        second: Second(E::V(OtherSize499)),
    };
    const OTHER_SIZE499_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(499))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(499))),
        },
        second: Second(E::V(OtherSize500)),
    };
    const OTHER_SIZE500_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(500))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(500))),
        },
        second: Second(E::V(OtherSize501)),
    };
    const OTHER_SIZE501_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(501))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(501))),
        },
        second: Second(E::V(OtherSize502)),
    };
    const OTHER_SIZE502_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(502))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(502))),
        },
        second: Second(E::V(OtherSize503)),
    };
    const OTHER_SIZE503_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(503))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(503))),
        },
        second: Second(E::V(OtherSize504)),
    };
    const OTHER_SIZE504_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(504))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(504))),
        },
        second: Second(E::V(OtherSize505)),
    };
    const OTHER_SIZE505_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(505))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(505))),
        },
        second: Second(E::V(OtherSize506)),
    };
    const OTHER_SIZE506_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(506))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(506))),
        },
        second: Second(E::V(OtherSize507)),
    };
    const OTHER_SIZE507_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(507))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(507))),
        },
        second: Second(E::V(OtherSize508)),
    };
    const OTHER_SIZE508_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(508))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(508))),
        },
        second: Second(E::V(OtherSize509)),
    };
    const OTHER_SIZE509_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(509))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(509))),
        },
        second: Second(E::V(OtherSize510)),
    };
    const OTHER_SIZE510_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(510))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(510))),
        },
        second: Second(E::V(OtherSize511)),
    };
    const OTHER_SIZE511_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(511))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(511))),
        },
        second: Second(E::V(OtherSize512)),
    };
    const OTHER_SIZE512_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(512))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(512))),
        },
        second: Second(E::V(OtherSize513)),
    };
    const OTHER_SIZE513_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(513))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(513))),
        },
        second: Second(E::V(OtherSize514)),
    };
    const OTHER_SIZE514_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(514))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(514))),
        },
        second: Second(E::V(OtherSize515)),
    };
    const OTHER_SIZE515_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(515))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(515))),
        },
        second: Second(E::V(OtherSize516)),
    };
    const OTHER_SIZE516_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(516))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(516))),
        },
        second: Second(E::V(OtherSize517)),
    };
    const OTHER_SIZE517_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(517))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(517))),
        },
        second: Second(E::V(OtherSize518)),
    };
    const OTHER_SIZE518_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(518))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(518))),
        },
        second: Second(E::V(OtherSize519)),
    };
    const OTHER_SIZE519_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(519))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(519))),
        },
        second: Second(E::V(OtherSize520)),
    };
    const OTHER_SIZE520_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(520))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(520))),
        },
        second: Second(E::V(OtherSize521)),
    };
    const OTHER_SIZE521_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(521))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(521))),
        },
        second: Second(E::V(OtherSize522)),
    };
    const OTHER_SIZE522_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(522))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(522))),
        },
        second: Second(E::V(OtherSize523)),
    };
    const OTHER_SIZE523_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(523))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(523))),
        },
        second: Second(E::V(OtherSize524)),
    };
    const OTHER_SIZE524_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(524))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(524))),
        },
        second: Second(E::V(OtherSize525)),
    };
    const OTHER_SIZE525_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(525))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(525))),
        },
        second: Second(E::V(OtherSize526)),
    };
    const OTHER_SIZE526_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(526))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(526))),
        },
        second: Second(E::V(OtherSize527)),
    };
    const OTHER_SIZE527_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(527))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(527))),
        },
        second: Second(E::V(OtherSize528)),
    };
    const OTHER_SIZE528_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(528))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(528))),
        },
        second: Second(E::V(OtherSize529)),
    };
    const OTHER_SIZE529_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(529))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(529))),
        },
        second: Second(E::V(OtherSize530)),
    };
    const OTHER_SIZE530_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(530))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(530))),
        },
        second: Second(E::V(OtherSize531)),
    };
    const OTHER_SIZE531_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(531))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(531))),
        },
        second: Second(E::V(OtherSize532)),
    };
    const OTHER_SIZE532_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(532))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(532))),
        },
        second: Second(E::V(OtherSize533)),
    };
    const OTHER_SIZE533_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(533))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(533))),
        },
        second: Second(E::V(OtherSize534)),
    };
    const OTHER_SIZE534_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(534))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(534))),
        },
        second: Second(E::V(OtherSize535)),
    };
    const OTHER_SIZE535_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(535))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(535))),
        },
        second: Second(E::V(OtherSize536)),
    };
    const OTHER_SIZE536_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(536))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(536))),
        },
        second: Second(E::V(OtherSize537)),
    };
    const OTHER_SIZE537_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(537))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(537))),
        },
        second: Second(E::V(OtherSize538)),
    };
    const OTHER_SIZE538_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(538))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(538))),
        },
        second: Second(E::V(OtherSize539)),
    };
    const OTHER_SIZE539_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(539))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(539))),
        },
        second: Second(E::V(OtherSize540)),
    };
    const OTHER_SIZE540_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(540))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(540))),
        },
        second: Second(E::V(OtherSize541)),
    };
    const OTHER_SIZE541_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(541))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(541))),
        },
        second: Second(E::V(OtherSize542)),
    };
    const OTHER_SIZE542_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(542))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(542))),
        },
        second: Second(E::V(OtherSize543)),
    };
    const OTHER_SIZE543_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(543))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(543))),
        },
        second: Second(E::V(OtherSize544)),
    };
    const OTHER_SIZE544_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(544))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(544))),
        },
        second: Second(E::V(OtherSize545)),
    };
    const OTHER_SIZE545_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(545))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(545))),
        },
        second: Second(E::V(OtherSize546)),
    };
    const OTHER_SIZE546_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(546))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(546))),
        },
        second: Second(E::V(OtherSize547)),
    };
    const OTHER_SIZE547_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(547))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(547))),
        },
        second: Second(E::V(OtherSize548)),
    };
    const OTHER_SIZE548_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(548))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(548))),
        },
        second: Second(E::V(OtherSize549)),
    };
    const OTHER_SIZE549_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(549))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(549))),
        },
        second: Second(E::V(OtherSize550)),
    };
    const OTHER_SIZE550_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(550))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(550))),
        },
        second: Second(E::V(OtherSize551)),
    };
    const OTHER_SIZE551_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(551))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(551))),
        },
        second: Second(E::V(OtherSize552)),
    };
    const OTHER_SIZE552_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(552))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(552))),
        },
        second: Second(E::V(OtherSize553)),
    };
    const OTHER_SIZE553_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(553))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(553))),
        },
        second: Second(E::V(OtherSize554)),
    };
    const OTHER_SIZE554_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(554))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(554))),
        },
        second: Second(E::V(OtherSize555)),
    };
    const OTHER_SIZE555_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(555))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(555))),
        },
        second: Second(E::V(OtherSize556)),
    };
    const OTHER_SIZE556_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(556))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(556))),
        },
        second: Second(E::V(OtherSize557)),
    };
    const OTHER_SIZE557_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(557))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(557))),
        },
        second: Second(E::V(OtherSize558)),
    };
    const OTHER_SIZE558_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(558))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(558))),
        },
        second: Second(E::V(OtherSize559)),
    };
    const OTHER_SIZE559_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(559))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(559))),
        },
        second: Second(E::V(OtherSize560)),
    };
    const OTHER_SIZE560_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(560))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(560))),
        },
        second: Second(E::V(OtherSize561)),
    };
    const OTHER_SIZE561_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(561))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(561))),
        },
        second: Second(E::V(OtherSize562)),
    };
    const OTHER_SIZE562_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(562))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(562))),
        },
        second: Second(E::V(OtherSize563)),
    };
    const OTHER_SIZE563_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(563))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(563))),
        },
        second: Second(E::V(OtherSize564)),
    };
    const OTHER_SIZE564_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(564))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(564))),
        },
        second: Second(E::V(OtherSize565)),
    };
    const OTHER_SIZE565_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(565))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(565))),
        },
        second: Second(E::V(OtherSize566)),
    };
    const OTHER_SIZE566_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(566))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(566))),
        },
        second: Second(E::V(OtherSize567)),
    };
    const OTHER_SIZE567_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(567))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(567))),
        },
        second: Second(E::V(OtherSize568)),
    };
    const OTHER_SIZE568_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(568))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(568))),
        },
        second: Second(E::V(OtherSize569)),
    };
    const OTHER_SIZE569_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(569))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(569))),
        },
        second: Second(E::V(OtherSize570)),
    };
    const OTHER_SIZE570_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(570))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(570))),
        },
        second: Second(E::V(OtherSize571)),
    };
    const OTHER_SIZE571_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(571))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(571))),
        },
        second: Second(E::V(OtherSize572)),
    };
    const OTHER_SIZE572_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(572))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(572))),
        },
        second: Second(E::V(OtherSize573)),
    };
    const OTHER_SIZE573_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(573))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(573))),
        },
        second: Second(E::V(OtherSize574)),
    };
    const OTHER_SIZE574_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(574))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(574))),
        },
        second: Second(E::V(OtherSize575)),
    };
    const OTHER_SIZE575_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(575))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(575))),
        },
        second: Second(E::V(OtherSize576)),
    };
    const OTHER_SIZE576_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(576))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(576))),
        },
        second: Second(E::V(OtherSize577)),
    };
    const OTHER_SIZE577_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(577))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(577))),
        },
        second: Second(E::V(OtherSize578)),
    };
    const OTHER_SIZE578_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(578))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(578))),
        },
        second: Second(E::V(OtherSize579)),
    };
    const OTHER_SIZE579_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(579))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(579))),
        },
        second: Second(E::V(OtherSize580)),
    };
    const OTHER_SIZE580_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(580))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(580))),
        },
        second: Second(E::V(OtherSize581)),
    };
    const OTHER_SIZE581_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(581))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(581))),
        },
        second: Second(E::V(OtherSize582)),
    };
    const OTHER_SIZE582_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(582))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(582))),
        },
        second: Second(E::V(OtherSize583)),
    };
    const OTHER_SIZE583_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(583))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(583))),
        },
        second: Second(E::V(OtherSize584)),
    };
    const OTHER_SIZE584_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(584))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(584))),
        },
        second: Second(E::V(OtherSize585)),
    };
    const OTHER_SIZE585_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(585))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(585))),
        },
        second: Second(E::V(OtherSize586)),
    };
    const OTHER_SIZE586_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(586))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(586))),
        },
        second: Second(E::V(OtherSize587)),
    };
    const OTHER_SIZE587_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(587))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(587))),
        },
        second: Second(E::V(OtherSize588)),
    };
    const OTHER_SIZE588_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(588))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(588))),
        },
        second: Second(E::V(OtherSize589)),
    };
    const OTHER_SIZE589_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(589))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(589))),
        },
        second: Second(E::V(OtherSize590)),
    };
    const OTHER_SIZE590_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(590))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(590))),
        },
        second: Second(E::V(OtherSize591)),
    };
    const OTHER_SIZE591_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(591))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(591))),
        },
        second: Second(E::V(OtherSize592)),
    };
    const OTHER_SIZE592_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(592))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(592))),
        },
        second: Second(E::V(OtherSize593)),
    };
    const OTHER_SIZE593_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(593))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(593))),
        },
        second: Second(E::V(OtherSize594)),
    };
    const OTHER_SIZE594_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(594))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(594))),
        },
        second: Second(E::V(OtherSize595)),
    };
    const OTHER_SIZE595_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(595))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(595))),
        },
        second: Second(E::V(OtherSize596)),
    };
    const OTHER_SIZE596_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(596))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(596))),
        },
        second: Second(E::V(OtherSize597)),
    };
    const OTHER_SIZE597_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(597))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(597))),
        },
        second: Second(E::V(OtherSize598)),
    };
    const OTHER_SIZE598_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(598))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(598))),
        },
        second: Second(E::V(OtherSize599)),
    };
    const OTHER_SIZE599_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(599))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(599))),
        },
        second: Second(E::V(OtherSize600)),
    };
    const OTHER_SIZE600_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(600))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(600))),
        },
        second: Second(E::V(OtherSize601)),
    };
    const OTHER_SIZE601_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(601))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(601))),
        },
        second: Second(E::V(OtherSize602)),
    };
    const OTHER_SIZE602_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(602))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(602))),
        },
        second: Second(E::V(OtherSize603)),
    };
    const OTHER_SIZE603_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(603))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(603))),
        },
        second: Second(E::V(OtherSize604)),
    };
    const OTHER_SIZE604_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(604))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(604))),
        },
        second: Second(E::V(OtherSize605)),
    };
    const OTHER_SIZE605_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(605))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(605))),
        },
        second: Second(E::V(OtherSize606)),
    };
    const OTHER_SIZE606_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(606))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(606))),
        },
        second: Second(E::V(OtherSize607)),
    };
    const OTHER_SIZE607_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(607))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(607))),
        },
        second: Second(E::V(OtherSize608)),
    };
    const OTHER_SIZE608_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(608))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(608))),
        },
        second: Second(E::V(OtherSize609)),
    };
    const OTHER_SIZE609_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(609))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(609))),
        },
        second: Second(E::V(OtherSize610)),
    };
    const OTHER_SIZE610_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(610))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(610))),
        },
        second: Second(E::V(OtherSize611)),
    };
    const OTHER_SIZE611_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(611))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(611))),
        },
        second: Second(E::V(OtherSize612)),
    };
    const OTHER_SIZE612_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(612))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(612))),
        },
        second: Second(E::V(OtherSize613)),
    };
    const OTHER_SIZE613_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(613))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(613))),
        },
        second: Second(E::V(OtherSize614)),
    };
    const OTHER_SIZE614_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(614))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(614))),
        },
        second: Second(E::V(OtherSize615)),
    };
    const OTHER_SIZE615_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(615))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(615))),
        },
        second: Second(E::V(OtherSize616)),
    };
    const OTHER_SIZE616_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(616))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(616))),
        },
        second: Second(E::V(OtherSize617)),
    };
    const OTHER_SIZE617_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(617))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(617))),
        },
        second: Second(E::V(OtherSize618)),
    };
    const OTHER_SIZE618_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(618))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(618))),
        },
        second: Second(E::V(OtherSize619)),
    };
    const OTHER_SIZE619_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(619))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(619))),
        },
        second: Second(E::V(OtherSize620)),
    };
    const OTHER_SIZE620_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(620))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(620))),
        },
        second: Second(E::V(OtherSize621)),
    };
    const OTHER_SIZE621_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(621))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(621))),
        },
        second: Second(E::V(OtherSize622)),
    };
    const OTHER_SIZE622_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(622))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(622))),
        },
        second: Second(E::V(OtherSize623)),
    };
    const OTHER_SIZE623_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(623))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(623))),
        },
        second: Second(E::V(OtherSize624)),
    };
    const OTHER_SIZE624_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(624))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(624))),
        },
        second: Second(E::V(OtherSize625)),
    };
    const OTHER_SIZE625_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(625))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(625))),
        },
        second: Second(E::V(OtherSize626)),
    };
    const OTHER_SIZE626_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(626))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(626))),
        },
        second: Second(E::V(OtherSize627)),
    };
    const OTHER_SIZE627_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(627))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(627))),
        },
        second: Second(E::V(OtherSize628)),
    };
    const OTHER_SIZE628_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(628))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(628))),
        },
        second: Second(E::V(OtherSize629)),
    };
    const OTHER_SIZE629_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(629))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(629))),
        },
        second: Second(E::V(OtherSize630)),
    };
    const OTHER_SIZE630_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(630))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(630))),
        },
        second: Second(E::V(OtherSize631)),
    };
    const OTHER_SIZE631_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(631))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(631))),
        },
        second: Second(E::V(OtherSize632)),
    };
    const OTHER_SIZE632_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(632))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(632))),
        },
        second: Second(E::V(OtherSize633)),
    };
    const OTHER_SIZE633_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(633))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(633))),
        },
        second: Second(E::V(OtherSize634)),
    };
    const OTHER_SIZE634_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(634))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(634))),
        },
        second: Second(E::V(OtherSize635)),
    };
    const OTHER_SIZE635_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(635))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(635))),
        },
        second: Second(E::V(OtherSize636)),
    };
    const OTHER_SIZE636_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(636))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(636))),
        },
        second: Second(E::V(OtherSize637)),
    };
    const OTHER_SIZE637_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(637))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(637))),
        },
        second: Second(E::V(OtherSize638)),
    };
    const OTHER_SIZE638_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(638))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(638))),
        },
        second: Second(E::V(OtherSize639)),
    };
    const OTHER_SIZE639_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(639))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(639))),
        },
        second: Second(E::V(OtherSize640)),
    };
    const OTHER_SIZE640_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(640))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(640))),
        },
        second: Second(E::V(OtherSize641)),
    };
    const OTHER_SIZE641_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(641))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(641))),
        },
        second: Second(E::V(OtherSize642)),
    };
    const OTHER_SIZE642_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(642))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(642))),
        },
        second: Second(E::V(OtherSize643)),
    };
    const OTHER_SIZE643_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(643))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(643))),
        },
        second: Second(E::V(OtherSize644)),
    };
    const OTHER_SIZE644_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(644))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(644))),
        },
        second: Second(E::V(OtherSize645)),
    };
    const OTHER_SIZE645_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(645))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(645))),
        },
        second: Second(E::V(OtherSize646)),
    };
    const OTHER_SIZE646_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(646))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(646))),
        },
        second: Second(E::V(OtherSize647)),
    };
    const OTHER_SIZE647_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(647))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(647))),
        },
        second: Second(E::V(OtherSize648)),
    };
    const OTHER_SIZE648_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(648))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(648))),
        },
        second: Second(E::V(OtherSize649)),
    };
    const OTHER_SIZE649_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(649))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(649))),
        },
        second: Second(E::V(OtherSize650)),
    };
    const OTHER_SIZE650_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(650))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(650))),
        },
        second: Second(E::V(OtherSize651)),
    };
    const OTHER_SIZE651_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(651))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(651))),
        },
        second: Second(E::V(OtherSize652)),
    };
    const OTHER_SIZE652_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(652))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(652))),
        },
        second: Second(E::V(OtherSize653)),
    };
    const OTHER_SIZE653_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(653))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(653))),
        },
        second: Second(E::V(OtherSize654)),
    };
    const OTHER_SIZE654_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(654))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(654))),
        },
        second: Second(E::V(OtherSize655)),
    };
    const OTHER_SIZE655_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(655))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(655))),
        },
        second: Second(E::V(OtherSize656)),
    };
    const OTHER_SIZE656_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(656))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(656))),
        },
        second: Second(E::V(OtherSize657)),
    };
    const OTHER_SIZE657_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(657))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(657))),
        },
        second: Second(E::V(OtherSize658)),
    };
    const OTHER_SIZE658_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(658))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(658))),
        },
        second: Second(E::V(OtherSize659)),
    };
    const OTHER_SIZE659_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(659))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(659))),
        },
        second: Second(E::V(OtherSize660)),
    };
    const OTHER_SIZE660_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(660))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(660))),
        },
        second: Second(E::V(OtherSize661)),
    };
    const OTHER_SIZE661_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(661))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(661))),
        },
        second: Second(E::V(OtherSize662)),
    };
    const OTHER_SIZE662_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(662))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(662))),
        },
        second: Second(E::V(OtherSize663)),
    };
    const OTHER_SIZE663_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(663))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(663))),
        },
        second: Second(E::V(OtherSize664)),
    };
    const OTHER_SIZE664_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(664))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(664))),
        },
        second: Second(E::V(OtherSize665)),
    };
    const OTHER_SIZE665_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(665))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(665))),
        },
        second: Second(E::V(OtherSize666)),
    };
    const OTHER_SIZE666_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(666))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(666))),
        },
        second: Second(E::V(OtherSize667)),
    };
    const OTHER_SIZE667_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(667))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(667))),
        },
        second: Second(E::V(OtherSize668)),
    };
    const OTHER_SIZE668_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(668))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(668))),
        },
        second: Second(E::V(OtherSize669)),
    };
    const OTHER_SIZE669_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(669))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(669))),
        },
        second: Second(E::V(OtherSize670)),
    };
    const OTHER_SIZE670_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(670))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(670))),
        },
        second: Second(E::V(OtherSize671)),
    };
    const OTHER_SIZE671_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(671))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(671))),
        },
        second: Second(E::V(OtherSize672)),
    };
    const OTHER_SIZE672_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(672))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(672))),
        },
        second: Second(E::V(OtherSize673)),
    };
    const OTHER_SIZE673_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(673))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(673))),
        },
        second: Second(E::V(OtherSize674)),
    };
    const OTHER_SIZE674_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(674))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(674))),
        },
        second: Second(E::V(OtherSize675)),
    };
    const OTHER_SIZE675_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(675))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(675))),
        },
        second: Second(E::V(OtherSize676)),
    };
    const OTHER_SIZE676_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(676))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(676))),
        },
        second: Second(E::V(OtherSize677)),
    };
    const OTHER_SIZE677_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(677))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(677))),
        },
        second: Second(E::V(OtherSize678)),
    };
    const OTHER_SIZE678_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(678))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(678))),
        },
        second: Second(E::V(OtherSize679)),
    };
    const OTHER_SIZE679_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(679))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(679))),
        },
        second: Second(E::V(OtherSize680)),
    };
    const OTHER_SIZE680_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(680))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(680))),
        },
        second: Second(E::V(OtherSize681)),
    };
    const OTHER_SIZE681_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(681))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(681))),
        },
        second: Second(E::V(OtherSize682)),
    };
    const OTHER_SIZE682_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(682))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(682))),
        },
        second: Second(E::V(OtherSize683)),
    };
    const OTHER_SIZE683_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(683))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(683))),
        },
        second: Second(E::V(OtherSize684)),
    };
    const OTHER_SIZE684_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(684))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(684))),
        },
        second: Second(E::V(OtherSize685)),
    };
    const OTHER_SIZE685_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(685))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(685))),
        },
        second: Second(E::V(OtherSize686)),
    };
    const OTHER_SIZE686_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(686))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(686))),
        },
        second: Second(E::V(OtherSize687)),
    };
    const OTHER_SIZE687_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(687))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(687))),
        },
        second: Second(E::V(OtherSize688)),
    };
    const OTHER_SIZE688_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(688))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(688))),
        },
        second: Second(E::V(OtherSize689)),
    };
    const OTHER_SIZE689_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(689))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(689))),
        },
        second: Second(E::V(OtherSize690)),
    };
    const OTHER_SIZE690_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(690))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(690))),
        },
        second: Second(E::V(OtherSize691)),
    };
    const OTHER_SIZE691_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(691))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(691))),
        },
        second: Second(E::V(OtherSize692)),
    };
    const OTHER_SIZE692_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(692))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(692))),
        },
        second: Second(E::V(OtherSize693)),
    };
    const OTHER_SIZE693_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(693))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(693))),
        },
        second: Second(E::V(OtherSize694)),
    };
    const OTHER_SIZE694_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(694))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(694))),
        },
        second: Second(E::V(OtherSize695)),
    };
    const OTHER_SIZE695_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(695))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(695))),
        },
        second: Second(E::V(OtherSize696)),
    };
    const OTHER_SIZE696_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(696))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(696))),
        },
        second: Second(E::V(OtherSize697)),
    };
    const OTHER_SIZE697_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(697))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(697))),
        },
        second: Second(E::V(OtherSize698)),
    };
    const OTHER_SIZE698_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(698))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(698))),
        },
        second: Second(E::V(OtherSize699)),
    };
    const OTHER_SIZE699_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(699))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(699))),
        },
        second: Second(E::V(OtherSize700)),
    };
    const OTHER_SIZE700_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(700))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(700))),
        },
        second: Second(E::V(OtherSize701)),
    };
    const OTHER_SIZE701_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(701))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(701))),
        },
        second: Second(E::V(OtherSize702)),
    };
    const OTHER_SIZE702_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(702))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(702))),
        },
        second: Second(E::V(OtherSize703)),
    };
    const OTHER_SIZE703_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(703))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(703))),
        },
        second: Second(E::V(OtherSize704)),
    };
    const OTHER_SIZE704_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(704))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(704))),
        },
        second: Second(E::V(OtherSize705)),
    };
    const OTHER_SIZE705_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(705))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(705))),
        },
        second: Second(E::V(OtherSize706)),
    };
    const OTHER_SIZE706_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(706))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(706))),
        },
        second: Second(E::V(OtherSize707)),
    };
    const OTHER_SIZE707_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(707))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(707))),
        },
        second: Second(E::V(OtherSize708)),
    };
    const OTHER_SIZE708_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(708))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(708))),
        },
        second: Second(E::V(OtherSize709)),
    };
    const OTHER_SIZE709_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(709))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(709))),
        },
        second: Second(E::V(OtherSize710)),
    };
    const OTHER_SIZE710_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(710))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(710))),
        },
        second: Second(E::V(OtherSize711)),
    };
    const OTHER_SIZE711_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(711))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(711))),
        },
        second: Second(E::V(OtherSize712)),
    };
    const OTHER_SIZE712_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(712))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(712))),
        },
        second: Second(E::V(OtherSize713)),
    };
    const OTHER_SIZE713_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(713))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(713))),
        },
        second: Second(E::V(OtherSize714)),
    };
    const OTHER_SIZE714_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(714))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(714))),
        },
        second: Second(E::V(OtherSize715)),
    };
    const OTHER_SIZE715_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(715))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(715))),
        },
        second: Second(E::V(OtherSize716)),
    };
    const OTHER_SIZE716_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(716))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(716))),
        },
        second: Second(E::V(OtherSize717)),
    };
    const OTHER_SIZE717_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(717))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(717))),
        },
        second: Second(E::V(OtherSize718)),
    };
    const OTHER_SIZE718_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(718))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(718))),
        },
        second: Second(E::V(OtherSize719)),
    };
    const OTHER_SIZE719_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(719))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(719))),
        },
        second: Second(E::V(OtherSize720)),
    };
    const OTHER_SIZE720_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(720))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(720))),
        },
        second: Second(E::V(OtherSize721)),
    };
    const OTHER_SIZE721_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(721))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(721))),
        },
        second: Second(E::V(OtherSize722)),
    };
    const OTHER_SIZE722_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(722))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(722))),
        },
        second: Second(E::V(OtherSize723)),
    };
    const OTHER_SIZE723_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(723))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(723))),
        },
        second: Second(E::V(OtherSize724)),
    };
    const OTHER_SIZE724_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(724))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(724))),
        },
        second: Second(E::V(OtherSize725)),
    };
    const OTHER_SIZE725_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(725))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(725))),
        },
        second: Second(E::V(OtherSize726)),
    };
    const OTHER_SIZE726_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(726))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(726))),
        },
        second: Second(E::V(OtherSize727)),
    };
    const OTHER_SIZE727_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(727))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(727))),
        },
        second: Second(E::V(OtherSize728)),
    };
    const OTHER_SIZE728_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(728))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(728))),
        },
        second: Second(E::V(OtherSize729)),
    };
    const OTHER_SIZE729_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(729))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(729))),
        },
        second: Second(E::V(OtherSize730)),
    };
    const OTHER_SIZE730_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(730))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(730))),
        },
        second: Second(E::V(OtherSize731)),
    };
    const OTHER_SIZE731_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(731))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(731))),
        },
        second: Second(E::V(OtherSize732)),
    };
    const OTHER_SIZE732_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(732))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(732))),
        },
        second: Second(E::V(OtherSize733)),
    };
    const OTHER_SIZE733_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(733))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(733))),
        },
        second: Second(E::V(OtherSize734)),
    };
    const OTHER_SIZE734_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(734))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(734))),
        },
        second: Second(E::V(OtherSize735)),
    };
    const OTHER_SIZE735_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(735))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(735))),
        },
        second: Second(E::V(OtherSize736)),
    };
    const OTHER_SIZE736_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(736))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(736))),
        },
        second: Second(E::V(OtherSize737)),
    };
    const OTHER_SIZE737_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(737))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(737))),
        },
        second: Second(E::V(OtherSize738)),
    };
    const OTHER_SIZE738_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(738))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(738))),
        },
        second: Second(E::V(OtherSize739)),
    };
    const OTHER_SIZE739_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(739))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(739))),
        },
        second: Second(E::V(OtherSize740)),
    };
    const OTHER_SIZE740_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(740))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(740))),
        },
        second: Second(E::V(OtherSize741)),
    };
    const OTHER_SIZE741_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(741))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(741))),
        },
        second: Second(E::V(OtherSize742)),
    };
    const OTHER_SIZE742_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(742))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(742))),
        },
        second: Second(E::V(OtherSize743)),
    };
    const OTHER_SIZE743_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(743))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(743))),
        },
        second: Second(E::V(OtherSize744)),
    };
    const OTHER_SIZE744_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(744))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(744))),
        },
        second: Second(E::V(OtherSize745)),
    };
    const OTHER_SIZE745_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(745))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(745))),
        },
        second: Second(E::V(OtherSize746)),
    };
    const OTHER_SIZE746_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(746))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(746))),
        },
        second: Second(E::V(OtherSize747)),
    };
    const OTHER_SIZE747_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(747))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(747))),
        },
        second: Second(E::V(OtherSize748)),
    };
    const OTHER_SIZE748_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(748))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(748))),
        },
        second: Second(E::V(OtherSize749)),
    };
    const OTHER_SIZE749_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(749))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(749))),
        },
        second: Second(E::V(OtherSize750)),
    };
    const OTHER_SIZE750_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(750))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(750))),
        },
        second: Second(E::V(OtherSize751)),
    };
    const OTHER_SIZE751_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(751))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(751))),
        },
        second: Second(E::V(OtherSize752)),
    };
    const OTHER_SIZE752_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(752))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(752))),
        },
        second: Second(E::V(OtherSize753)),
    };
    const OTHER_SIZE753_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(753))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(753))),
        },
        second: Second(E::V(OtherSize754)),
    };
    const OTHER_SIZE754_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(754))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(754))),
        },
        second: Second(E::V(OtherSize755)),
    };
    const OTHER_SIZE755_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(755))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(755))),
        },
        second: Second(E::V(OtherSize756)),
    };
    const OTHER_SIZE756_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(756))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(756))),
        },
        second: Second(E::V(OtherSize757)),
    };
    const OTHER_SIZE757_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(757))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(757))),
        },
        second: Second(E::V(OtherSize758)),
    };
    const OTHER_SIZE758_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(758))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(758))),
        },
        second: Second(E::V(OtherSize759)),
    };
    const OTHER_SIZE759_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(759))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(759))),
        },
        second: Second(E::V(OtherSize760)),
    };
    const OTHER_SIZE760_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(760))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(760))),
        },
        second: Second(E::V(OtherSize761)),
    };
    const OTHER_SIZE761_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(761))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(761))),
        },
        second: Second(E::V(OtherSize762)),
    };
    const OTHER_SIZE762_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(762))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(762))),
        },
        second: Second(E::V(OtherSize763)),
    };
    const OTHER_SIZE763_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(763))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(763))),
        },
        second: Second(E::V(OtherSize764)),
    };
    const OTHER_SIZE764_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(764))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(764))),
        },
        second: Second(E::V(OtherSize765)),
    };
    const OTHER_SIZE765_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(765))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(765))),
        },
        second: Second(E::V(OtherSize766)),
    };
    const OTHER_SIZE766_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(766))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(766))),
        },
        second: Second(E::V(OtherSize767)),
    };
    const OTHER_SIZE767_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(767))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(767))),
        },
        second: Second(E::V(OtherSize768)),
    };
    const OTHER_SIZE768_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(768))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(768))),
        },
        second: Second(E::V(OtherSize769)),
    };
    const OTHER_SIZE769_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(769))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(769))),
        },
        second: Second(E::V(OtherSize770)),
    };
    const OTHER_SIZE770_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(770))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(770))),
        },
        second: Second(E::V(OtherSize771)),
    };
    const OTHER_SIZE771_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(771))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(771))),
        },
        second: Second(E::V(OtherSize772)),
    };
    const OTHER_SIZE772_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(772))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(772))),
        },
        second: Second(E::V(OtherSize773)),
    };
    const OTHER_SIZE773_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(773))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(773))),
        },
        second: Second(E::V(OtherSize774)),
    };
    const OTHER_SIZE774_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(774))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(774))),
        },
        second: Second(E::V(OtherSize775)),
    };
    const OTHER_SIZE775_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(775))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(775))),
        },
        second: Second(E::V(OtherSize776)),
    };
    const OTHER_SIZE776_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(776))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(776))),
        },
        second: Second(E::V(OtherSize777)),
    };
    const OTHER_SIZE777_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(777))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(777))),
        },
        second: Second(E::V(OtherSize778)),
    };
    const OTHER_SIZE778_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(778))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(778))),
        },
        second: Second(E::V(OtherSize779)),
    };
    const OTHER_SIZE779_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(779))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(779))),
        },
        second: Second(E::V(OtherSize780)),
    };
    const OTHER_SIZE780_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(780))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(780))),
        },
        second: Second(E::V(OtherSize781)),
    };
    const OTHER_SIZE781_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(781))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(781))),
        },
        second: Second(E::V(OtherSize782)),
    };
    const OTHER_SIZE782_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(782))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(782))),
        },
        second: Second(E::V(OtherSize783)),
    };
    const OTHER_SIZE783_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(783))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(783))),
        },
        second: Second(E::V(OtherSize784)),
    };
    const OTHER_SIZE784_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(784))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(784))),
        },
        second: Second(E::V(OtherSize785)),
    };
    const OTHER_SIZE785_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(785))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(785))),
        },
        second: Second(E::V(OtherSize786)),
    };
    const OTHER_SIZE786_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(786))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(786))),
        },
        second: Second(E::V(OtherSize787)),
    };
    const OTHER_SIZE787_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(787))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(787))),
        },
        second: Second(E::V(OtherSize788)),
    };
    const OTHER_SIZE788_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(788))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(788))),
        },
        second: Second(E::V(OtherSize789)),
    };
    const OTHER_SIZE789_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(789))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(789))),
        },
        second: Second(E::V(OtherSize790)),
    };
    const OTHER_SIZE790_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(790))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(790))),
        },
        second: Second(E::V(OtherSize791)),
    };
    const OTHER_SIZE791_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(791))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(791))),
        },
        second: Second(E::V(OtherSize792)),
    };
    const OTHER_SIZE792_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(792))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(792))),
        },
        second: Second(E::V(OtherSize793)),
    };
    const OTHER_SIZE793_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(793))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(793))),
        },
        second: Second(E::V(OtherSize794)),
    };
    const OTHER_SIZE794_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(794))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(794))),
        },
        second: Second(E::V(OtherSize795)),
    };
    const OTHER_SIZE795_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(795))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(795))),
        },
        second: Second(E::V(OtherSize796)),
    };
    const OTHER_SIZE796_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(796))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(796))),
        },
        second: Second(E::V(OtherSize797)),
    };
    const OTHER_SIZE797_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(797))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(797))),
        },
        second: Second(E::V(OtherSize798)),
    };
    const OTHER_SIZE798_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(798))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(798))),
        },
        second: Second(E::V(OtherSize799)),
    };
    const OTHER_SIZE799_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(799))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(799))),
        },
        second: Second(E::V(OtherSize800)),
    };
    const OTHER_SIZE800_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(800))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(800))),
        },
        second: Second(E::V(OtherSize801)),
    };
    const OTHER_SIZE801_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(801))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(801))),
        },
        second: Second(E::V(OtherSize802)),
    };
    const OTHER_SIZE802_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(802))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(802))),
        },
        second: Second(E::V(OtherSize803)),
    };
    const OTHER_SIZE803_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(803))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(803))),
        },
        second: Second(E::V(OtherSize804)),
    };
    const OTHER_SIZE804_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(804))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(804))),
        },
        second: Second(E::V(OtherSize805)),
    };
    const OTHER_SIZE805_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(805))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(805))),
        },
        second: Second(E::V(OtherSize806)),
    };
    const OTHER_SIZE806_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(806))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(806))),
        },
        second: Second(E::V(OtherSize807)),
    };
    const OTHER_SIZE807_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(807))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(807))),
        },
        second: Second(E::V(OtherSize808)),
    };
    const OTHER_SIZE808_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(808))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(808))),
        },
        second: Second(E::V(OtherSize809)),
    };
    const OTHER_SIZE809_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(809))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(809))),
        },
        second: Second(E::V(OtherSize810)),
    };
    const OTHER_SIZE810_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(810))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(810))),
        },
        second: Second(E::V(OtherSize811)),
    };
    const OTHER_SIZE811_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(811))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(811))),
        },
        second: Second(E::V(OtherSize812)),
    };
    const OTHER_SIZE812_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(812))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(812))),
        },
        second: Second(E::V(OtherSize813)),
    };
    const OTHER_SIZE813_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(813))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(813))),
        },
        second: Second(E::V(OtherSize814)),
    };
    const OTHER_SIZE814_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(814))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(814))),
        },
        second: Second(E::V(OtherSize815)),
    };
    const OTHER_SIZE815_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(815))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(815))),
        },
        second: Second(E::V(OtherSize816)),
    };
    const OTHER_SIZE816_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(816))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(816))),
        },
        second: Second(E::V(OtherSize817)),
    };
    const OTHER_SIZE817_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(817))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(817))),
        },
        second: Second(E::V(OtherSize818)),
    };
    const OTHER_SIZE818_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(818))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(818))),
        },
        second: Second(E::V(OtherSize819)),
    };
    const OTHER_SIZE819_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(819))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(819))),
        },
        second: Second(E::V(OtherSize820)),
    };
    const OTHER_SIZE820_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(820))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(820))),
        },
        second: Second(E::V(OtherSize821)),
    };
    const OTHER_SIZE821_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(821))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(821))),
        },
        second: Second(E::V(OtherSize822)),
    };
    const OTHER_SIZE822_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(822))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(822))),
        },
        second: Second(E::V(OtherSize823)),
    };
    const OTHER_SIZE823_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(823))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(823))),
        },
        second: Second(E::V(OtherSize824)),
    };
    const OTHER_SIZE824_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(824))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(824))),
        },
        second: Second(E::V(OtherSize825)),
    };
    const OTHER_SIZE825_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(825))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(825))),
        },
        second: Second(E::V(OtherSize826)),
    };
    const OTHER_SIZE826_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(826))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(826))),
        },
        second: Second(E::V(OtherSize827)),
    };
    const OTHER_SIZE827_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(827))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(827))),
        },
        second: Second(E::V(OtherSize828)),
    };
    const OTHER_SIZE828_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(828))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(828))),
        },
        second: Second(E::V(OtherSize829)),
    };
    const OTHER_SIZE829_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(829))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(829))),
        },
        second: Second(E::V(OtherSize830)),
    };
    const OTHER_SIZE830_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(830))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(830))),
        },
        second: Second(E::V(OtherSize831)),
    };
    const OTHER_SIZE831_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(831))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(831))),
        },
        second: Second(E::V(OtherSize832)),
    };
    const OTHER_SIZE832_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(832))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(832))),
        },
        second: Second(E::V(OtherSize833)),
    };
    const OTHER_SIZE833_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(833))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(833))),
        },
        second: Second(E::V(OtherSize834)),
    };
    const OTHER_SIZE834_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(834))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(834))),
        },
        second: Second(E::V(OtherSize835)),
    };
    const OTHER_SIZE835_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(835))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(835))),
        },
        second: Second(E::V(OtherSize836)),
    };
    const OTHER_SIZE836_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(836))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(836))),
        },
        second: Second(E::V(OtherSize837)),
    };
    const OTHER_SIZE837_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(837))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(837))),
        },
        second: Second(E::V(OtherSize838)),
    };
    const OTHER_SIZE838_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(838))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(838))),
        },
        second: Second(E::V(OtherSize839)),
    };
    const OTHER_SIZE839_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(839))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(839))),
        },
        second: Second(E::V(OtherSize840)),
    };
    const OTHER_SIZE840_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(840))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(840))),
        },
        second: Second(E::V(OtherSize841)),
    };
    const OTHER_SIZE841_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(841))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(841))),
        },
        second: Second(E::V(OtherSize842)),
    };
    const OTHER_SIZE842_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(842))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(842))),
        },
        second: Second(E::V(OtherSize843)),
    };
    const OTHER_SIZE843_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(843))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(843))),
        },
        second: Second(E::V(OtherSize844)),
    };
    const OTHER_SIZE844_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(844))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(844))),
        },
        second: Second(E::V(OtherSize845)),
    };
    const OTHER_SIZE845_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(845))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(845))),
        },
        second: Second(E::V(OtherSize846)),
    };
    const OTHER_SIZE846_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(846))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(846))),
        },
        second: Second(E::V(OtherSize847)),
    };
    const OTHER_SIZE847_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(847))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(847))),
        },
        second: Second(E::V(OtherSize848)),
    };
    const OTHER_SIZE848_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(848))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(848))),
        },
        second: Second(E::V(OtherSize849)),
    };
    const OTHER_SIZE849_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(849))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(849))),
        },
        second: Second(E::V(OtherSize850)),
    };
    const OTHER_SIZE850_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(850))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(850))),
        },
        second: Second(E::V(OtherSize851)),
    };
    const OTHER_SIZE851_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(851))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(851))),
        },
        second: Second(E::V(OtherSize852)),
    };
    const OTHER_SIZE852_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(852))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(852))),
        },
        second: Second(E::V(OtherSize853)),
    };
    const OTHER_SIZE853_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(853))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(853))),
        },
        second: Second(E::V(OtherSize854)),
    };
    const OTHER_SIZE854_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(854))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(854))),
        },
        second: Second(E::V(OtherSize855)),
    };
    const OTHER_SIZE855_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(855))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(855))),
        },
        second: Second(E::V(OtherSize856)),
    };
    const OTHER_SIZE856_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(856))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(856))),
        },
        second: Second(E::V(OtherSize857)),
    };
    const OTHER_SIZE857_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(857))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(857))),
        },
        second: Second(E::V(OtherSize858)),
    };
    const OTHER_SIZE858_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(858))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(858))),
        },
        second: Second(E::V(OtherSize859)),
    };
    const OTHER_SIZE859_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(859))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(859))),
        },
        second: Second(E::V(OtherSize860)),
    };
    const OTHER_SIZE860_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(860))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(860))),
        },
        second: Second(E::V(OtherSize861)),
    };
    const OTHER_SIZE861_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(861))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(861))),
        },
        second: Second(E::V(OtherSize862)),
    };
    const OTHER_SIZE862_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(862))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(862))),
        },
        second: Second(E::V(OtherSize863)),
    };
    const OTHER_SIZE863_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(863))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(863))),
        },
        second: Second(E::V(OtherSize864)),
    };
    const OTHER_SIZE864_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(864))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(864))),
        },
        second: Second(E::V(OtherSize865)),
    };
    const OTHER_SIZE865_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(865))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(865))),
        },
        second: Second(E::V(OtherSize866)),
    };
    const OTHER_SIZE866_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(866))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(866))),
        },
        second: Second(E::V(OtherSize867)),
    };
    const OTHER_SIZE867_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(867))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(867))),
        },
        second: Second(E::V(OtherSize868)),
    };
    const OTHER_SIZE868_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(868))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(868))),
        },
        second: Second(E::V(OtherSize869)),
    };
    const OTHER_SIZE869_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(869))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(869))),
        },
        second: Second(E::V(OtherSize870)),
    };
    const OTHER_SIZE870_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(870))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(870))),
        },
        second: Second(E::V(OtherSize871)),
    };
    const OTHER_SIZE871_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(871))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(871))),
        },
        second: Second(E::V(OtherSize872)),
    };
    const OTHER_SIZE872_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(872))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(872))),
        },
        second: Second(E::V(OtherSize873)),
    };
    const OTHER_SIZE873_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(873))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(873))),
        },
        second: Second(E::V(OtherSize874)),
    };
    const OTHER_SIZE874_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(874))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(874))),
        },
        second: Second(E::V(OtherSize875)),
    };
    const OTHER_SIZE875_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(875))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(875))),
        },
        second: Second(E::V(OtherSize876)),
    };
    const OTHER_SIZE876_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(876))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(876))),
        },
        second: Second(E::V(OtherSize877)),
    };
    const OTHER_SIZE877_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(877))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(877))),
        },
        second: Second(E::V(OtherSize878)),
    };
    const OTHER_SIZE878_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(878))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(878))),
        },
        second: Second(E::V(OtherSize879)),
    };
    const OTHER_SIZE879_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(879))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(879))),
        },
        second: Second(E::V(OtherSize880)),
    };
    const OTHER_SIZE880_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(880))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(880))),
        },
        second: Second(E::V(OtherSize881)),
    };
    const OTHER_SIZE881_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(881))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(881))),
        },
        second: Second(E::V(OtherSize882)),
    };
    const OTHER_SIZE882_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(882))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(882))),
        },
        second: Second(E::V(OtherSize883)),
    };
    const OTHER_SIZE883_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(883))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(883))),
        },
        second: Second(E::V(OtherSize884)),
    };
    const OTHER_SIZE884_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(884))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(884))),
        },
        second: Second(E::V(OtherSize885)),
    };
    const OTHER_SIZE885_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(885))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(885))),
        },
        second: Second(E::V(OtherSize886)),
    };
    const OTHER_SIZE886_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(886))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(886))),
        },
        second: Second(E::V(OtherSize887)),
    };
    const OTHER_SIZE887_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(887))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(887))),
        },
        second: Second(E::V(OtherSize888)),
    };
    const OTHER_SIZE888_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(888))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(888))),
        },
        second: Second(E::V(OtherSize889)),
    };
    const OTHER_SIZE889_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(889))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(889))),
        },
        second: Second(E::V(OtherSize890)),
    };
    const OTHER_SIZE890_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(890))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(890))),
        },
        second: Second(E::V(OtherSize891)),
    };
    const OTHER_SIZE891_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(891))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(891))),
        },
        second: Second(E::V(OtherSize892)),
    };
    const OTHER_SIZE892_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(892))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(892))),
        },
        second: Second(E::V(OtherSize893)),
    };
    const OTHER_SIZE893_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(893))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(893))),
        },
        second: Second(E::V(OtherSize894)),
    };
    const OTHER_SIZE894_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(894))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(894))),
        },
        second: Second(E::V(OtherSize895)),
    };
    const OTHER_SIZE895_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(895))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(895))),
        },
        second: Second(E::V(OtherSize896)),
    };
    const OTHER_SIZE896_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(896))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(896))),
        },
        second: Second(E::V(OtherSize897)),
    };
    const OTHER_SIZE897_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(897))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(897))),
        },
        second: Second(E::V(OtherSize898)),
    };
    const OTHER_SIZE898_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(898))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(898))),
        },
        second: Second(E::V(OtherSize899)),
    };
    const OTHER_SIZE899_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(899))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(899))),
        },
        second: Second(E::V(OtherSize900)),
    };
    const OTHER_SIZE900_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(900))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(900))),
        },
        second: Second(E::V(OtherSize901)),
    };
    const OTHER_SIZE901_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(901))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(901))),
        },
        second: Second(E::V(OtherSize902)),
    };
    const OTHER_SIZE902_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(902))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(902))),
        },
        second: Second(E::V(OtherSize903)),
    };
    const OTHER_SIZE903_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(903))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(903))),
        },
        second: Second(E::V(OtherSize904)),
    };
    const OTHER_SIZE904_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(904))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(904))),
        },
        second: Second(E::V(OtherSize905)),
    };
    const OTHER_SIZE905_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(905))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(905))),
        },
        second: Second(E::V(OtherSize906)),
    };
    const OTHER_SIZE906_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(906))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(906))),
        },
        second: Second(E::V(OtherSize907)),
    };
    const OTHER_SIZE907_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(907))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(907))),
        },
        second: Second(E::V(OtherSize908)),
    };
    const OTHER_SIZE908_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(908))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(908))),
        },
        second: Second(E::V(OtherSize909)),
    };
    const OTHER_SIZE909_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(909))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(909))),
        },
        second: Second(E::V(OtherSize910)),
    };
    const OTHER_SIZE910_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(910))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(910))),
        },
        second: Second(E::V(OtherSize911)),
    };
    const OTHER_SIZE911_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(911))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(911))),
        },
        second: Second(E::V(OtherSize912)),
    };
    const OTHER_SIZE912_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(912))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(912))),
        },
        second: Second(E::V(OtherSize913)),
    };
    const OTHER_SIZE913_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(913))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(913))),
        },
        second: Second(E::V(OtherSize914)),
    };
    const OTHER_SIZE914_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(914))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(914))),
        },
        second: Second(E::V(OtherSize915)),
    };
    const OTHER_SIZE915_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(915))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(915))),
        },
        second: Second(E::V(OtherSize916)),
    };
    const OTHER_SIZE916_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(916))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(916))),
        },
        second: Second(E::V(OtherSize917)),
    };
    const OTHER_SIZE917_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(917))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(917))),
        },
        second: Second(E::V(OtherSize918)),
    };
    const OTHER_SIZE918_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(918))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(918))),
        },
        second: Second(E::V(OtherSize919)),
    };
    const OTHER_SIZE919_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(919))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(919))),
        },
        second: Second(E::V(OtherSize920)),
    };
    const OTHER_SIZE920_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(920))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(920))),
        },
        second: Second(E::V(OtherSize921)),
    };
    const OTHER_SIZE921_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(921))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(921))),
        },
        second: Second(E::V(OtherSize922)),
    };
    const OTHER_SIZE922_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(922))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(922))),
        },
        second: Second(E::V(OtherSize923)),
    };
    const OTHER_SIZE923_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(923))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(923))),
        },
        second: Second(E::V(OtherSize924)),
    };
    const OTHER_SIZE924_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(924))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(924))),
        },
        second: Second(E::V(OtherSize925)),
    };
    const OTHER_SIZE925_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(925))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(925))),
        },
        second: Second(E::V(OtherSize926)),
    };
    const OTHER_SIZE926_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(926))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(926))),
        },
        second: Second(E::V(OtherSize927)),
    };
    const OTHER_SIZE927_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(927))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(927))),
        },
        second: Second(E::V(OtherSize928)),
    };
    const OTHER_SIZE928_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(928))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(928))),
        },
        second: Second(E::V(OtherSize929)),
    };
    const OTHER_SIZE929_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(929))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(929))),
        },
        second: Second(E::V(OtherSize930)),
    };
    const OTHER_SIZE930_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(930))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(930))),
        },
        second: Second(E::V(OtherSize931)),
    };
    const OTHER_SIZE931_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(931))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(931))),
        },
        second: Second(E::V(OtherSize932)),
    };
    const OTHER_SIZE932_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(932))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(932))),
        },
        second: Second(E::V(OtherSize933)),
    };
    const OTHER_SIZE933_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(933))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(933))),
        },
        second: Second(E::V(OtherSize934)),
    };
    const OTHER_SIZE934_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(934))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(934))),
        },
        second: Second(E::V(OtherSize935)),
    };
    const OTHER_SIZE935_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(935))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(935))),
        },
        second: Second(E::V(OtherSize936)),
    };
    const OTHER_SIZE936_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(936))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(936))),
        },
        second: Second(E::V(OtherSize937)),
    };
    const OTHER_SIZE937_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(937))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(937))),
        },
        second: Second(E::V(OtherSize938)),
    };
    const OTHER_SIZE938_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(938))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(938))),
        },
        second: Second(E::V(OtherSize939)),
    };
    const OTHER_SIZE939_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(939))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(939))),
        },
        second: Second(E::V(OtherSize940)),
    };
    const OTHER_SIZE940_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(940))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(940))),
        },
        second: Second(E::V(OtherSize941)),
    };
    const OTHER_SIZE941_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(941))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(941))),
        },
        second: Second(E::V(OtherSize942)),
    };
    const OTHER_SIZE942_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(942))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(942))),
        },
        second: Second(E::V(OtherSize943)),
    };
    const OTHER_SIZE943_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(943))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(943))),
        },
        second: Second(E::V(OtherSize944)),
    };
    const OTHER_SIZE944_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(944))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(944))),
        },
        second: Second(E::V(OtherSize945)),
    };
    const OTHER_SIZE945_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(945))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(945))),
        },
        second: Second(E::V(OtherSize946)),
    };
    const OTHER_SIZE946_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(946))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(946))),
        },
        second: Second(E::V(OtherSize947)),
    };
    const OTHER_SIZE947_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(947))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(947))),
        },
        second: Second(E::V(OtherSize948)),
    };
    const OTHER_SIZE948_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(948))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(948))),
        },
        second: Second(E::V(OtherSize949)),
    };
    const OTHER_SIZE949_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(949))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(949))),
        },
        second: Second(E::V(OtherSize950)),
    };
    const OTHER_SIZE950_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(950))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(950))),
        },
        second: Second(E::V(OtherSize951)),
    };
    const OTHER_SIZE951_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(951))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(951))),
        },
        second: Second(E::V(OtherSize952)),
    };
    const OTHER_SIZE952_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(952))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(952))),
        },
        second: Second(E::V(OtherSize953)),
    };
    const OTHER_SIZE953_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(953))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(953))),
        },
        second: Second(E::V(OtherSize954)),
    };
    const OTHER_SIZE954_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(954))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(954))),
        },
        second: Second(E::V(OtherSize955)),
    };
    const OTHER_SIZE955_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(955))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(955))),
        },
        second: Second(E::V(OtherSize956)),
    };
    const OTHER_SIZE956_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(956))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(956))),
        },
        second: Second(E::V(OtherSize957)),
    };
    const OTHER_SIZE957_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(957))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(957))),
        },
        second: Second(E::V(OtherSize958)),
    };
    const OTHER_SIZE958_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(958))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(958))),
        },
        second: Second(E::V(OtherSize959)),
    };
    const OTHER_SIZE959_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(959))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(959))),
        },
        second: Second(E::V(OtherSize960)),
    };
    const OTHER_SIZE960_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(960))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(960))),
        },
        second: Second(E::V(OtherSize961)),
    };
    const OTHER_SIZE961_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(961))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(961))),
        },
        second: Second(E::V(OtherSize962)),
    };
    const OTHER_SIZE962_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(962))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(962))),
        },
        second: Second(E::V(OtherSize963)),
    };
    const OTHER_SIZE963_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(963))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(963))),
        },
        second: Second(E::V(OtherSize964)),
    };
    const OTHER_SIZE964_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(964))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(964))),
        },
        second: Second(E::V(OtherSize965)),
    };
    const OTHER_SIZE965_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(965))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(965))),
        },
        second: Second(E::V(OtherSize966)),
    };
    const OTHER_SIZE966_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(966))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(966))),
        },
        second: Second(E::V(OtherSize967)),
    };
    const OTHER_SIZE967_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(967))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(967))),
        },
        second: Second(E::V(OtherSize968)),
    };
    const OTHER_SIZE968_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(968))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(968))),
        },
        second: Second(E::V(OtherSize969)),
    };
    const OTHER_SIZE969_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(969))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(969))),
        },
        second: Second(E::V(OtherSize970)),
    };
    const OTHER_SIZE970_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(970))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(970))),
        },
        second: Second(E::V(OtherSize971)),
    };
    const OTHER_SIZE971_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(971))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(971))),
        },
        second: Second(E::V(OtherSize972)),
    };
    const OTHER_SIZE972_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(972))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(972))),
        },
        second: Second(E::V(OtherSize973)),
    };
    const OTHER_SIZE973_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(973))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(973))),
        },
        second: Second(E::V(OtherSize974)),
    };
    const OTHER_SIZE974_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(974))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(974))),
        },
        second: Second(E::V(OtherSize975)),
    };
    const OTHER_SIZE975_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(975))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(975))),
        },
        second: Second(E::V(OtherSize976)),
    };
    const OTHER_SIZE976_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(976))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(976))),
        },
        second: Second(E::V(OtherSize977)),
    };
    const OTHER_SIZE977_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(977))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(977))),
        },
        second: Second(E::V(OtherSize978)),
    };
    const OTHER_SIZE978_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(978))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(978))),
        },
        second: Second(E::V(OtherSize979)),
    };
    const OTHER_SIZE979_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(979))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(979))),
        },
        second: Second(E::V(OtherSize980)),
    };
    const OTHER_SIZE980_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(980))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(980))),
        },
        second: Second(E::V(OtherSize981)),
    };
    const OTHER_SIZE981_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(981))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(981))),
        },
        second: Second(E::V(OtherSize982)),
    };
    const OTHER_SIZE982_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(982))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(982))),
        },
        second: Second(E::V(OtherSize983)),
    };
    const OTHER_SIZE983_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(983))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(983))),
        },
        second: Second(E::V(OtherSize984)),
    };
    const OTHER_SIZE984_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(984))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(984))),
        },
        second: Second(E::V(OtherSize985)),
    };
    const OTHER_SIZE985_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(985))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(985))),
        },
        second: Second(E::V(OtherSize986)),
    };
    const OTHER_SIZE986_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(986))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(986))),
        },
        second: Second(E::V(OtherSize987)),
    };
    const OTHER_SIZE987_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(987))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(987))),
        },
        second: Second(E::V(OtherSize988)),
    };
    const OTHER_SIZE988_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(988))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(988))),
        },
        second: Second(E::V(OtherSize989)),
    };
    const OTHER_SIZE989_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(989))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(989))),
        },
        second: Second(E::V(OtherSize990)),
    };
    const OTHER_SIZE990_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(990))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(990))),
        },
        second: Second(E::V(OtherSize991)),
    };
    const OTHER_SIZE991_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(991))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(991))),
        },
        second: Second(E::V(OtherSize992)),
    };
    const OTHER_SIZE992_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(992))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(992))),
        },
        second: Second(E::V(OtherSize993)),
    };
    const OTHER_SIZE993_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(993))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(993))),
        },
        second: Second(E::V(OtherSize994)),
    };
    const OTHER_SIZE994_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(994))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(994))),
        },
        second: Second(E::V(OtherSize995)),
    };
    const OTHER_SIZE995_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(995))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(995))),
        },
        second: Second(E::V(OtherSize996)),
    };
    const OTHER_SIZE996_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(996))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(996))),
        },
        second: Second(E::V(OtherSize997)),
    };
    const OTHER_SIZE997_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(997))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(997))),
        },
        second: Second(E::V(OtherSize998)),
    };
    const OTHER_SIZE998_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(998))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(998))),
        },
        second: Second(E::V(OtherSize999)),
    };
    const OTHER_SIZE999_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(999))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(999))),
        },
        second: Second(E::V(OtherSize1000)),
    };
    const OTHER_SIZE1000_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1000))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1000))),
        },
        second: Second(E::V(OtherSize1001)),
    };
    const OTHER_SIZE1001_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1001))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1001))),
        },
        second: Second(E::V(OtherSize1002)),
    };
    const OTHER_SIZE1002_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1002))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1002))),
        },
        second: Second(E::V(OtherSize1003)),
    };
    const OTHER_SIZE1003_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1003))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1003))),
        },
        second: Second(E::V(OtherSize1004)),
    };
    const OTHER_SIZE1004_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1004))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1004))),
        },
        second: Second(E::V(OtherSize1005)),
    };
    const OTHER_SIZE1005_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1005))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1005))),
        },
        second: Second(E::V(OtherSize1006)),
    };
    const OTHER_SIZE1006_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1006))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1006))),
        },
        second: Second(E::V(OtherSize1007)),
    };
    const OTHER_SIZE1007_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1007))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1007))),
        },
        second: Second(E::V(OtherSize1008)),
    };
    const OTHER_SIZE1008_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1008))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1008))),
        },
        second: Second(E::V(OtherSize1009)),
    };
    const OTHER_SIZE1009_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1009))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1009))),
        },
        second: Second(E::V(OtherSize1010)),
    };
    const OTHER_SIZE1010_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1010))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1010))),
        },
        second: Second(E::V(OtherSize1011)),
    };
    const OTHER_SIZE1011_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1011))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1011))),
        },
        second: Second(E::V(OtherSize1012)),
    };
    const OTHER_SIZE1012_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1012))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1012))),
        },
        second: Second(E::V(OtherSize1013)),
    };
    const OTHER_SIZE1013_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1013))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1013))),
        },
        second: Second(E::V(OtherSize1014)),
    };
    const OTHER_SIZE1014_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1014))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1014))),
        },
        second: Second(E::V(OtherSize1015)),
    };
    const OTHER_SIZE1015_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1015))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1015))),
        },
        second: Second(E::V(OtherSize1016)),
    };
    const OTHER_SIZE1016_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1016))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1016))),
        },
        second: Second(E::V(OtherSize1017)),
    };
    const OTHER_SIZE1017_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1017))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1017))),
        },
        second: Second(E::V(OtherSize1018)),
    };
    const OTHER_SIZE1018_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1018))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1018))),
        },
        second: Second(E::V(OtherSize1019)),
    };
    const OTHER_SIZE1019_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1019))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1019))),
        },
        second: Second(E::V(OtherSize1020)),
    };
    const OTHER_SIZE1020_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1020))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1020))),
        },
        second: Second(E::V(OtherSize1021)),
    };
    const OTHER_SIZE1021_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1021))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1021))),
        },
        second: Second(E::V(OtherSize1022)),
    };
    const OTHER_SIZE1022_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1022))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1022))),
        },
        second: Second(E::V(OtherSize1023)),
    };
    const OTHER_SIZE1023_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1023))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1023))),
        },
        second: Second(E::V(OtherSize1024)),
    };
    const OTHER_SIZE1024_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(LEU32(1024))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(1024))),
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
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const U32_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(4))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    const U128_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Metasymbol(Any(8))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
}

impl<'a> Rules<U8SliceTerminal<'a>, WavVariable> for WavRules {
    fn get(&self, variable: &WavVariable) -> Option<&RightRule<U8SliceTerminal<'a>, WavVariable>> {
        Some(match variable {
            Wav => &Self::WAV_RIGHT_RULE,
            Chunks => &Self::CHUNKS_RIGHT_RULE,

            Chunk => &Self::CHUNK_RIGHT_RULE,
            Chunk2 => &Self::CHUNK2_RIGHT_RULE,

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
            OtherSize2 => &Self::OTHER_SIZE2_RIGHT_RULE,
            OtherSize3 => &Self::OTHER_SIZE3_RIGHT_RULE,
            OtherSize4 => &Self::OTHER_SIZE4_RIGHT_RULE,
            OtherSize5 => &Self::OTHER_SIZE5_RIGHT_RULE,
            OtherSize6 => &Self::OTHER_SIZE6_RIGHT_RULE,
            OtherSize7 => &Self::OTHER_SIZE7_RIGHT_RULE,
            OtherSize8 => &Self::OTHER_SIZE8_RIGHT_RULE,
            OtherSize9 => &Self::OTHER_SIZE9_RIGHT_RULE,
            OtherSize10 => &Self::OTHER_SIZE10_RIGHT_RULE,
            OtherSize11 => &Self::OTHER_SIZE11_RIGHT_RULE,
            OtherSize12 => &Self::OTHER_SIZE12_RIGHT_RULE,
            OtherSize13 => &Self::OTHER_SIZE13_RIGHT_RULE,
            OtherSize14 => &Self::OTHER_SIZE14_RIGHT_RULE,
            OtherSize15 => &Self::OTHER_SIZE15_RIGHT_RULE,
            OtherSize16 => &Self::OTHER_SIZE16_RIGHT_RULE,
            OtherSize17 => &Self::OTHER_SIZE17_RIGHT_RULE,
            OtherSize18 => &Self::OTHER_SIZE18_RIGHT_RULE,
            OtherSize19 => &Self::OTHER_SIZE19_RIGHT_RULE,
            OtherSize20 => &Self::OTHER_SIZE20_RIGHT_RULE,
            OtherSize21 => &Self::OTHER_SIZE21_RIGHT_RULE,
            OtherSize22 => &Self::OTHER_SIZE22_RIGHT_RULE,
            OtherSize23 => &Self::OTHER_SIZE23_RIGHT_RULE,
            OtherSize24 => &Self::OTHER_SIZE24_RIGHT_RULE,
            OtherSize25 => &Self::OTHER_SIZE25_RIGHT_RULE,
            OtherSize26 => &Self::OTHER_SIZE26_RIGHT_RULE,
            OtherSize27 => &Self::OTHER_SIZE27_RIGHT_RULE,
            OtherSize28 => &Self::OTHER_SIZE28_RIGHT_RULE,
            OtherSize29 => &Self::OTHER_SIZE29_RIGHT_RULE,
            OtherSize30 => &Self::OTHER_SIZE30_RIGHT_RULE,
            OtherSize31 => &Self::OTHER_SIZE31_RIGHT_RULE,
            OtherSize32 => &Self::OTHER_SIZE32_RIGHT_RULE,
            OtherSize33 => &Self::OTHER_SIZE33_RIGHT_RULE,
            OtherSize34 => &Self::OTHER_SIZE34_RIGHT_RULE,
            OtherSize35 => &Self::OTHER_SIZE35_RIGHT_RULE,
            OtherSize36 => &Self::OTHER_SIZE36_RIGHT_RULE,
            OtherSize37 => &Self::OTHER_SIZE37_RIGHT_RULE,
            OtherSize38 => &Self::OTHER_SIZE38_RIGHT_RULE,
            OtherSize39 => &Self::OTHER_SIZE39_RIGHT_RULE,
            OtherSize40 => &Self::OTHER_SIZE40_RIGHT_RULE,
            OtherSize41 => &Self::OTHER_SIZE41_RIGHT_RULE,
            OtherSize42 => &Self::OTHER_SIZE42_RIGHT_RULE,
            OtherSize43 => &Self::OTHER_SIZE43_RIGHT_RULE,
            OtherSize44 => &Self::OTHER_SIZE44_RIGHT_RULE,
            OtherSize45 => &Self::OTHER_SIZE45_RIGHT_RULE,
            OtherSize46 => &Self::OTHER_SIZE46_RIGHT_RULE,
            OtherSize47 => &Self::OTHER_SIZE47_RIGHT_RULE,
            OtherSize48 => &Self::OTHER_SIZE48_RIGHT_RULE,
            OtherSize49 => &Self::OTHER_SIZE49_RIGHT_RULE,
            OtherSize50 => &Self::OTHER_SIZE50_RIGHT_RULE,
            OtherSize51 => &Self::OTHER_SIZE51_RIGHT_RULE,
            OtherSize52 => &Self::OTHER_SIZE52_RIGHT_RULE,
            OtherSize53 => &Self::OTHER_SIZE53_RIGHT_RULE,
            OtherSize54 => &Self::OTHER_SIZE54_RIGHT_RULE,
            OtherSize55 => &Self::OTHER_SIZE55_RIGHT_RULE,
            OtherSize56 => &Self::OTHER_SIZE56_RIGHT_RULE,
            OtherSize57 => &Self::OTHER_SIZE57_RIGHT_RULE,
            OtherSize58 => &Self::OTHER_SIZE58_RIGHT_RULE,
            OtherSize59 => &Self::OTHER_SIZE59_RIGHT_RULE,
            OtherSize60 => &Self::OTHER_SIZE60_RIGHT_RULE,
            OtherSize61 => &Self::OTHER_SIZE61_RIGHT_RULE,
            OtherSize62 => &Self::OTHER_SIZE62_RIGHT_RULE,
            OtherSize63 => &Self::OTHER_SIZE63_RIGHT_RULE,
            OtherSize64 => &Self::OTHER_SIZE64_RIGHT_RULE,
            OtherSize65 => &Self::OTHER_SIZE65_RIGHT_RULE,
            OtherSize66 => &Self::OTHER_SIZE66_RIGHT_RULE,
            OtherSize67 => &Self::OTHER_SIZE67_RIGHT_RULE,
            OtherSize68 => &Self::OTHER_SIZE68_RIGHT_RULE,
            OtherSize69 => &Self::OTHER_SIZE69_RIGHT_RULE,
            OtherSize70 => &Self::OTHER_SIZE70_RIGHT_RULE,
            OtherSize71 => &Self::OTHER_SIZE71_RIGHT_RULE,
            OtherSize72 => &Self::OTHER_SIZE72_RIGHT_RULE,
            OtherSize73 => &Self::OTHER_SIZE73_RIGHT_RULE,
            OtherSize74 => &Self::OTHER_SIZE74_RIGHT_RULE,
            OtherSize75 => &Self::OTHER_SIZE75_RIGHT_RULE,
            OtherSize76 => &Self::OTHER_SIZE76_RIGHT_RULE,
            OtherSize77 => &Self::OTHER_SIZE77_RIGHT_RULE,
            OtherSize78 => &Self::OTHER_SIZE78_RIGHT_RULE,
            OtherSize79 => &Self::OTHER_SIZE79_RIGHT_RULE,
            OtherSize80 => &Self::OTHER_SIZE80_RIGHT_RULE,
            OtherSize81 => &Self::OTHER_SIZE81_RIGHT_RULE,
            OtherSize82 => &Self::OTHER_SIZE82_RIGHT_RULE,
            OtherSize83 => &Self::OTHER_SIZE83_RIGHT_RULE,
            OtherSize84 => &Self::OTHER_SIZE84_RIGHT_RULE,
            OtherSize85 => &Self::OTHER_SIZE85_RIGHT_RULE,
            OtherSize86 => &Self::OTHER_SIZE86_RIGHT_RULE,
            OtherSize87 => &Self::OTHER_SIZE87_RIGHT_RULE,
            OtherSize88 => &Self::OTHER_SIZE88_RIGHT_RULE,
            OtherSize89 => &Self::OTHER_SIZE89_RIGHT_RULE,
            OtherSize90 => &Self::OTHER_SIZE90_RIGHT_RULE,
            OtherSize91 => &Self::OTHER_SIZE91_RIGHT_RULE,
            OtherSize92 => &Self::OTHER_SIZE92_RIGHT_RULE,
            OtherSize93 => &Self::OTHER_SIZE93_RIGHT_RULE,
            OtherSize94 => &Self::OTHER_SIZE94_RIGHT_RULE,
            OtherSize95 => &Self::OTHER_SIZE95_RIGHT_RULE,
            OtherSize96 => &Self::OTHER_SIZE96_RIGHT_RULE,
            OtherSize97 => &Self::OTHER_SIZE97_RIGHT_RULE,
            OtherSize98 => &Self::OTHER_SIZE98_RIGHT_RULE,
            OtherSize99 => &Self::OTHER_SIZE99_RIGHT_RULE,
            OtherSize100 => &Self::OTHER_SIZE100_RIGHT_RULE,
            OtherSize101 => &Self::OTHER_SIZE101_RIGHT_RULE,
            OtherSize102 => &Self::OTHER_SIZE102_RIGHT_RULE,
            OtherSize103 => &Self::OTHER_SIZE103_RIGHT_RULE,
            OtherSize104 => &Self::OTHER_SIZE104_RIGHT_RULE,
            OtherSize105 => &Self::OTHER_SIZE105_RIGHT_RULE,
            OtherSize106 => &Self::OTHER_SIZE106_RIGHT_RULE,
            OtherSize107 => &Self::OTHER_SIZE107_RIGHT_RULE,
            OtherSize108 => &Self::OTHER_SIZE108_RIGHT_RULE,
            OtherSize109 => &Self::OTHER_SIZE109_RIGHT_RULE,
            OtherSize110 => &Self::OTHER_SIZE110_RIGHT_RULE,
            OtherSize111 => &Self::OTHER_SIZE111_RIGHT_RULE,
            OtherSize112 => &Self::OTHER_SIZE112_RIGHT_RULE,
            OtherSize113 => &Self::OTHER_SIZE113_RIGHT_RULE,
            OtherSize114 => &Self::OTHER_SIZE114_RIGHT_RULE,
            OtherSize115 => &Self::OTHER_SIZE115_RIGHT_RULE,
            OtherSize116 => &Self::OTHER_SIZE116_RIGHT_RULE,
            OtherSize117 => &Self::OTHER_SIZE117_RIGHT_RULE,
            OtherSize118 => &Self::OTHER_SIZE118_RIGHT_RULE,
            OtherSize119 => &Self::OTHER_SIZE119_RIGHT_RULE,
            OtherSize120 => &Self::OTHER_SIZE120_RIGHT_RULE,
            OtherSize121 => &Self::OTHER_SIZE121_RIGHT_RULE,
            OtherSize122 => &Self::OTHER_SIZE122_RIGHT_RULE,
            OtherSize123 => &Self::OTHER_SIZE123_RIGHT_RULE,
            OtherSize124 => &Self::OTHER_SIZE124_RIGHT_RULE,
            OtherSize125 => &Self::OTHER_SIZE125_RIGHT_RULE,
            OtherSize126 => &Self::OTHER_SIZE126_RIGHT_RULE,
            OtherSize127 => &Self::OTHER_SIZE127_RIGHT_RULE,
            OtherSize128 => &Self::OTHER_SIZE128_RIGHT_RULE,
            OtherSize129 => &Self::OTHER_SIZE129_RIGHT_RULE,
            OtherSize130 => &Self::OTHER_SIZE130_RIGHT_RULE,
            OtherSize131 => &Self::OTHER_SIZE131_RIGHT_RULE,
            OtherSize132 => &Self::OTHER_SIZE132_RIGHT_RULE,
            OtherSize133 => &Self::OTHER_SIZE133_RIGHT_RULE,
            OtherSize134 => &Self::OTHER_SIZE134_RIGHT_RULE,
            OtherSize135 => &Self::OTHER_SIZE135_RIGHT_RULE,
            OtherSize136 => &Self::OTHER_SIZE136_RIGHT_RULE,
            OtherSize137 => &Self::OTHER_SIZE137_RIGHT_RULE,
            OtherSize138 => &Self::OTHER_SIZE138_RIGHT_RULE,
            OtherSize139 => &Self::OTHER_SIZE139_RIGHT_RULE,
            OtherSize140 => &Self::OTHER_SIZE140_RIGHT_RULE,
            OtherSize141 => &Self::OTHER_SIZE141_RIGHT_RULE,
            OtherSize142 => &Self::OTHER_SIZE142_RIGHT_RULE,
            OtherSize143 => &Self::OTHER_SIZE143_RIGHT_RULE,
            OtherSize144 => &Self::OTHER_SIZE144_RIGHT_RULE,
            OtherSize145 => &Self::OTHER_SIZE145_RIGHT_RULE,
            OtherSize146 => &Self::OTHER_SIZE146_RIGHT_RULE,
            OtherSize147 => &Self::OTHER_SIZE147_RIGHT_RULE,
            OtherSize148 => &Self::OTHER_SIZE148_RIGHT_RULE,
            OtherSize149 => &Self::OTHER_SIZE149_RIGHT_RULE,
            OtherSize150 => &Self::OTHER_SIZE150_RIGHT_RULE,
            OtherSize151 => &Self::OTHER_SIZE151_RIGHT_RULE,
            OtherSize152 => &Self::OTHER_SIZE152_RIGHT_RULE,
            OtherSize153 => &Self::OTHER_SIZE153_RIGHT_RULE,
            OtherSize154 => &Self::OTHER_SIZE154_RIGHT_RULE,
            OtherSize155 => &Self::OTHER_SIZE155_RIGHT_RULE,
            OtherSize156 => &Self::OTHER_SIZE156_RIGHT_RULE,
            OtherSize157 => &Self::OTHER_SIZE157_RIGHT_RULE,
            OtherSize158 => &Self::OTHER_SIZE158_RIGHT_RULE,
            OtherSize159 => &Self::OTHER_SIZE159_RIGHT_RULE,
            OtherSize160 => &Self::OTHER_SIZE160_RIGHT_RULE,
            OtherSize161 => &Self::OTHER_SIZE161_RIGHT_RULE,
            OtherSize162 => &Self::OTHER_SIZE162_RIGHT_RULE,
            OtherSize163 => &Self::OTHER_SIZE163_RIGHT_RULE,
            OtherSize164 => &Self::OTHER_SIZE164_RIGHT_RULE,
            OtherSize165 => &Self::OTHER_SIZE165_RIGHT_RULE,
            OtherSize166 => &Self::OTHER_SIZE166_RIGHT_RULE,
            OtherSize167 => &Self::OTHER_SIZE167_RIGHT_RULE,
            OtherSize168 => &Self::OTHER_SIZE168_RIGHT_RULE,
            OtherSize169 => &Self::OTHER_SIZE169_RIGHT_RULE,
            OtherSize170 => &Self::OTHER_SIZE170_RIGHT_RULE,
            OtherSize171 => &Self::OTHER_SIZE171_RIGHT_RULE,
            OtherSize172 => &Self::OTHER_SIZE172_RIGHT_RULE,
            OtherSize173 => &Self::OTHER_SIZE173_RIGHT_RULE,
            OtherSize174 => &Self::OTHER_SIZE174_RIGHT_RULE,
            OtherSize175 => &Self::OTHER_SIZE175_RIGHT_RULE,
            OtherSize176 => &Self::OTHER_SIZE176_RIGHT_RULE,
            OtherSize177 => &Self::OTHER_SIZE177_RIGHT_RULE,
            OtherSize178 => &Self::OTHER_SIZE178_RIGHT_RULE,
            OtherSize179 => &Self::OTHER_SIZE179_RIGHT_RULE,
            OtherSize180 => &Self::OTHER_SIZE180_RIGHT_RULE,
            OtherSize181 => &Self::OTHER_SIZE181_RIGHT_RULE,
            OtherSize182 => &Self::OTHER_SIZE182_RIGHT_RULE,
            OtherSize183 => &Self::OTHER_SIZE183_RIGHT_RULE,
            OtherSize184 => &Self::OTHER_SIZE184_RIGHT_RULE,
            OtherSize185 => &Self::OTHER_SIZE185_RIGHT_RULE,
            OtherSize186 => &Self::OTHER_SIZE186_RIGHT_RULE,
            OtherSize187 => &Self::OTHER_SIZE187_RIGHT_RULE,
            OtherSize188 => &Self::OTHER_SIZE188_RIGHT_RULE,
            OtherSize189 => &Self::OTHER_SIZE189_RIGHT_RULE,
            OtherSize190 => &Self::OTHER_SIZE190_RIGHT_RULE,
            OtherSize191 => &Self::OTHER_SIZE191_RIGHT_RULE,
            OtherSize192 => &Self::OTHER_SIZE192_RIGHT_RULE,
            OtherSize193 => &Self::OTHER_SIZE193_RIGHT_RULE,
            OtherSize194 => &Self::OTHER_SIZE194_RIGHT_RULE,
            OtherSize195 => &Self::OTHER_SIZE195_RIGHT_RULE,
            OtherSize196 => &Self::OTHER_SIZE196_RIGHT_RULE,
            OtherSize197 => &Self::OTHER_SIZE197_RIGHT_RULE,
            OtherSize198 => &Self::OTHER_SIZE198_RIGHT_RULE,
            OtherSize199 => &Self::OTHER_SIZE199_RIGHT_RULE,
            OtherSize200 => &Self::OTHER_SIZE200_RIGHT_RULE,
            OtherSize201 => &Self::OTHER_SIZE201_RIGHT_RULE,
            OtherSize202 => &Self::OTHER_SIZE202_RIGHT_RULE,
            OtherSize203 => &Self::OTHER_SIZE203_RIGHT_RULE,
            OtherSize204 => &Self::OTHER_SIZE204_RIGHT_RULE,
            OtherSize205 => &Self::OTHER_SIZE205_RIGHT_RULE,
            OtherSize206 => &Self::OTHER_SIZE206_RIGHT_RULE,
            OtherSize207 => &Self::OTHER_SIZE207_RIGHT_RULE,
            OtherSize208 => &Self::OTHER_SIZE208_RIGHT_RULE,
            OtherSize209 => &Self::OTHER_SIZE209_RIGHT_RULE,
            OtherSize210 => &Self::OTHER_SIZE210_RIGHT_RULE,
            OtherSize211 => &Self::OTHER_SIZE211_RIGHT_RULE,
            OtherSize212 => &Self::OTHER_SIZE212_RIGHT_RULE,
            OtherSize213 => &Self::OTHER_SIZE213_RIGHT_RULE,
            OtherSize214 => &Self::OTHER_SIZE214_RIGHT_RULE,
            OtherSize215 => &Self::OTHER_SIZE215_RIGHT_RULE,
            OtherSize216 => &Self::OTHER_SIZE216_RIGHT_RULE,
            OtherSize217 => &Self::OTHER_SIZE217_RIGHT_RULE,
            OtherSize218 => &Self::OTHER_SIZE218_RIGHT_RULE,
            OtherSize219 => &Self::OTHER_SIZE219_RIGHT_RULE,
            OtherSize220 => &Self::OTHER_SIZE220_RIGHT_RULE,
            OtherSize221 => &Self::OTHER_SIZE221_RIGHT_RULE,
            OtherSize222 => &Self::OTHER_SIZE222_RIGHT_RULE,
            OtherSize223 => &Self::OTHER_SIZE223_RIGHT_RULE,
            OtherSize224 => &Self::OTHER_SIZE224_RIGHT_RULE,
            OtherSize225 => &Self::OTHER_SIZE225_RIGHT_RULE,
            OtherSize226 => &Self::OTHER_SIZE226_RIGHT_RULE,
            OtherSize227 => &Self::OTHER_SIZE227_RIGHT_RULE,
            OtherSize228 => &Self::OTHER_SIZE228_RIGHT_RULE,
            OtherSize229 => &Self::OTHER_SIZE229_RIGHT_RULE,
            OtherSize230 => &Self::OTHER_SIZE230_RIGHT_RULE,
            OtherSize231 => &Self::OTHER_SIZE231_RIGHT_RULE,
            OtherSize232 => &Self::OTHER_SIZE232_RIGHT_RULE,
            OtherSize233 => &Self::OTHER_SIZE233_RIGHT_RULE,
            OtherSize234 => &Self::OTHER_SIZE234_RIGHT_RULE,
            OtherSize235 => &Self::OTHER_SIZE235_RIGHT_RULE,
            OtherSize236 => &Self::OTHER_SIZE236_RIGHT_RULE,
            OtherSize237 => &Self::OTHER_SIZE237_RIGHT_RULE,
            OtherSize238 => &Self::OTHER_SIZE238_RIGHT_RULE,
            OtherSize239 => &Self::OTHER_SIZE239_RIGHT_RULE,
            OtherSize240 => &Self::OTHER_SIZE240_RIGHT_RULE,
            OtherSize241 => &Self::OTHER_SIZE241_RIGHT_RULE,
            OtherSize242 => &Self::OTHER_SIZE242_RIGHT_RULE,
            OtherSize243 => &Self::OTHER_SIZE243_RIGHT_RULE,
            OtherSize244 => &Self::OTHER_SIZE244_RIGHT_RULE,
            OtherSize245 => &Self::OTHER_SIZE245_RIGHT_RULE,
            OtherSize246 => &Self::OTHER_SIZE246_RIGHT_RULE,
            OtherSize247 => &Self::OTHER_SIZE247_RIGHT_RULE,
            OtherSize248 => &Self::OTHER_SIZE248_RIGHT_RULE,
            OtherSize249 => &Self::OTHER_SIZE249_RIGHT_RULE,
            OtherSize250 => &Self::OTHER_SIZE250_RIGHT_RULE,
            OtherSize251 => &Self::OTHER_SIZE251_RIGHT_RULE,
            OtherSize252 => &Self::OTHER_SIZE252_RIGHT_RULE,
            OtherSize253 => &Self::OTHER_SIZE253_RIGHT_RULE,
            OtherSize254 => &Self::OTHER_SIZE254_RIGHT_RULE,
            OtherSize255 => &Self::OTHER_SIZE255_RIGHT_RULE,
            OtherSize256 => &Self::OTHER_SIZE256_RIGHT_RULE,
            OtherSize257 => &Self::OTHER_SIZE257_RIGHT_RULE,
            OtherSize258 => &Self::OTHER_SIZE258_RIGHT_RULE,
            OtherSize259 => &Self::OTHER_SIZE259_RIGHT_RULE,
            OtherSize260 => &Self::OTHER_SIZE260_RIGHT_RULE,
            OtherSize261 => &Self::OTHER_SIZE261_RIGHT_RULE,
            OtherSize262 => &Self::OTHER_SIZE262_RIGHT_RULE,
            OtherSize263 => &Self::OTHER_SIZE263_RIGHT_RULE,
            OtherSize264 => &Self::OTHER_SIZE264_RIGHT_RULE,
            OtherSize265 => &Self::OTHER_SIZE265_RIGHT_RULE,
            OtherSize266 => &Self::OTHER_SIZE266_RIGHT_RULE,
            OtherSize267 => &Self::OTHER_SIZE267_RIGHT_RULE,
            OtherSize268 => &Self::OTHER_SIZE268_RIGHT_RULE,
            OtherSize269 => &Self::OTHER_SIZE269_RIGHT_RULE,
            OtherSize270 => &Self::OTHER_SIZE270_RIGHT_RULE,
            OtherSize271 => &Self::OTHER_SIZE271_RIGHT_RULE,
            OtherSize272 => &Self::OTHER_SIZE272_RIGHT_RULE,
            OtherSize273 => &Self::OTHER_SIZE273_RIGHT_RULE,
            OtherSize274 => &Self::OTHER_SIZE274_RIGHT_RULE,
            OtherSize275 => &Self::OTHER_SIZE275_RIGHT_RULE,
            OtherSize276 => &Self::OTHER_SIZE276_RIGHT_RULE,
            OtherSize277 => &Self::OTHER_SIZE277_RIGHT_RULE,
            OtherSize278 => &Self::OTHER_SIZE278_RIGHT_RULE,
            OtherSize279 => &Self::OTHER_SIZE279_RIGHT_RULE,
            OtherSize280 => &Self::OTHER_SIZE280_RIGHT_RULE,
            OtherSize281 => &Self::OTHER_SIZE281_RIGHT_RULE,
            OtherSize282 => &Self::OTHER_SIZE282_RIGHT_RULE,
            OtherSize283 => &Self::OTHER_SIZE283_RIGHT_RULE,
            OtherSize284 => &Self::OTHER_SIZE284_RIGHT_RULE,
            OtherSize285 => &Self::OTHER_SIZE285_RIGHT_RULE,
            OtherSize286 => &Self::OTHER_SIZE286_RIGHT_RULE,
            OtherSize287 => &Self::OTHER_SIZE287_RIGHT_RULE,
            OtherSize288 => &Self::OTHER_SIZE288_RIGHT_RULE,
            OtherSize289 => &Self::OTHER_SIZE289_RIGHT_RULE,
            OtherSize290 => &Self::OTHER_SIZE290_RIGHT_RULE,
            OtherSize291 => &Self::OTHER_SIZE291_RIGHT_RULE,
            OtherSize292 => &Self::OTHER_SIZE292_RIGHT_RULE,
            OtherSize293 => &Self::OTHER_SIZE293_RIGHT_RULE,
            OtherSize294 => &Self::OTHER_SIZE294_RIGHT_RULE,
            OtherSize295 => &Self::OTHER_SIZE295_RIGHT_RULE,
            OtherSize296 => &Self::OTHER_SIZE296_RIGHT_RULE,
            OtherSize297 => &Self::OTHER_SIZE297_RIGHT_RULE,
            OtherSize298 => &Self::OTHER_SIZE298_RIGHT_RULE,
            OtherSize299 => &Self::OTHER_SIZE299_RIGHT_RULE,
            OtherSize300 => &Self::OTHER_SIZE300_RIGHT_RULE,
            OtherSize301 => &Self::OTHER_SIZE301_RIGHT_RULE,
            OtherSize302 => &Self::OTHER_SIZE302_RIGHT_RULE,
            OtherSize303 => &Self::OTHER_SIZE303_RIGHT_RULE,
            OtherSize304 => &Self::OTHER_SIZE304_RIGHT_RULE,
            OtherSize305 => &Self::OTHER_SIZE305_RIGHT_RULE,
            OtherSize306 => &Self::OTHER_SIZE306_RIGHT_RULE,
            OtherSize307 => &Self::OTHER_SIZE307_RIGHT_RULE,
            OtherSize308 => &Self::OTHER_SIZE308_RIGHT_RULE,
            OtherSize309 => &Self::OTHER_SIZE309_RIGHT_RULE,
            OtherSize310 => &Self::OTHER_SIZE310_RIGHT_RULE,
            OtherSize311 => &Self::OTHER_SIZE311_RIGHT_RULE,
            OtherSize312 => &Self::OTHER_SIZE312_RIGHT_RULE,
            OtherSize313 => &Self::OTHER_SIZE313_RIGHT_RULE,
            OtherSize314 => &Self::OTHER_SIZE314_RIGHT_RULE,
            OtherSize315 => &Self::OTHER_SIZE315_RIGHT_RULE,
            OtherSize316 => &Self::OTHER_SIZE316_RIGHT_RULE,
            OtherSize317 => &Self::OTHER_SIZE317_RIGHT_RULE,
            OtherSize318 => &Self::OTHER_SIZE318_RIGHT_RULE,
            OtherSize319 => &Self::OTHER_SIZE319_RIGHT_RULE,
            OtherSize320 => &Self::OTHER_SIZE320_RIGHT_RULE,
            OtherSize321 => &Self::OTHER_SIZE321_RIGHT_RULE,
            OtherSize322 => &Self::OTHER_SIZE322_RIGHT_RULE,
            OtherSize323 => &Self::OTHER_SIZE323_RIGHT_RULE,
            OtherSize324 => &Self::OTHER_SIZE324_RIGHT_RULE,
            OtherSize325 => &Self::OTHER_SIZE325_RIGHT_RULE,
            OtherSize326 => &Self::OTHER_SIZE326_RIGHT_RULE,
            OtherSize327 => &Self::OTHER_SIZE327_RIGHT_RULE,
            OtherSize328 => &Self::OTHER_SIZE328_RIGHT_RULE,
            OtherSize329 => &Self::OTHER_SIZE329_RIGHT_RULE,
            OtherSize330 => &Self::OTHER_SIZE330_RIGHT_RULE,
            OtherSize331 => &Self::OTHER_SIZE331_RIGHT_RULE,
            OtherSize332 => &Self::OTHER_SIZE332_RIGHT_RULE,
            OtherSize333 => &Self::OTHER_SIZE333_RIGHT_RULE,
            OtherSize334 => &Self::OTHER_SIZE334_RIGHT_RULE,
            OtherSize335 => &Self::OTHER_SIZE335_RIGHT_RULE,
            OtherSize336 => &Self::OTHER_SIZE336_RIGHT_RULE,
            OtherSize337 => &Self::OTHER_SIZE337_RIGHT_RULE,
            OtherSize338 => &Self::OTHER_SIZE338_RIGHT_RULE,
            OtherSize339 => &Self::OTHER_SIZE339_RIGHT_RULE,
            OtherSize340 => &Self::OTHER_SIZE340_RIGHT_RULE,
            OtherSize341 => &Self::OTHER_SIZE341_RIGHT_RULE,
            OtherSize342 => &Self::OTHER_SIZE342_RIGHT_RULE,
            OtherSize343 => &Self::OTHER_SIZE343_RIGHT_RULE,
            OtherSize344 => &Self::OTHER_SIZE344_RIGHT_RULE,
            OtherSize345 => &Self::OTHER_SIZE345_RIGHT_RULE,
            OtherSize346 => &Self::OTHER_SIZE346_RIGHT_RULE,
            OtherSize347 => &Self::OTHER_SIZE347_RIGHT_RULE,
            OtherSize348 => &Self::OTHER_SIZE348_RIGHT_RULE,
            OtherSize349 => &Self::OTHER_SIZE349_RIGHT_RULE,
            OtherSize350 => &Self::OTHER_SIZE350_RIGHT_RULE,
            OtherSize351 => &Self::OTHER_SIZE351_RIGHT_RULE,
            OtherSize352 => &Self::OTHER_SIZE352_RIGHT_RULE,
            OtherSize353 => &Self::OTHER_SIZE353_RIGHT_RULE,
            OtherSize354 => &Self::OTHER_SIZE354_RIGHT_RULE,
            OtherSize355 => &Self::OTHER_SIZE355_RIGHT_RULE,
            OtherSize356 => &Self::OTHER_SIZE356_RIGHT_RULE,
            OtherSize357 => &Self::OTHER_SIZE357_RIGHT_RULE,
            OtherSize358 => &Self::OTHER_SIZE358_RIGHT_RULE,
            OtherSize359 => &Self::OTHER_SIZE359_RIGHT_RULE,
            OtherSize360 => &Self::OTHER_SIZE360_RIGHT_RULE,
            OtherSize361 => &Self::OTHER_SIZE361_RIGHT_RULE,
            OtherSize362 => &Self::OTHER_SIZE362_RIGHT_RULE,
            OtherSize363 => &Self::OTHER_SIZE363_RIGHT_RULE,
            OtherSize364 => &Self::OTHER_SIZE364_RIGHT_RULE,
            OtherSize365 => &Self::OTHER_SIZE365_RIGHT_RULE,
            OtherSize366 => &Self::OTHER_SIZE366_RIGHT_RULE,
            OtherSize367 => &Self::OTHER_SIZE367_RIGHT_RULE,
            OtherSize368 => &Self::OTHER_SIZE368_RIGHT_RULE,
            OtherSize369 => &Self::OTHER_SIZE369_RIGHT_RULE,
            OtherSize370 => &Self::OTHER_SIZE370_RIGHT_RULE,
            OtherSize371 => &Self::OTHER_SIZE371_RIGHT_RULE,
            OtherSize372 => &Self::OTHER_SIZE372_RIGHT_RULE,
            OtherSize373 => &Self::OTHER_SIZE373_RIGHT_RULE,
            OtherSize374 => &Self::OTHER_SIZE374_RIGHT_RULE,
            OtherSize375 => &Self::OTHER_SIZE375_RIGHT_RULE,
            OtherSize376 => &Self::OTHER_SIZE376_RIGHT_RULE,
            OtherSize377 => &Self::OTHER_SIZE377_RIGHT_RULE,
            OtherSize378 => &Self::OTHER_SIZE378_RIGHT_RULE,
            OtherSize379 => &Self::OTHER_SIZE379_RIGHT_RULE,
            OtherSize380 => &Self::OTHER_SIZE380_RIGHT_RULE,
            OtherSize381 => &Self::OTHER_SIZE381_RIGHT_RULE,
            OtherSize382 => &Self::OTHER_SIZE382_RIGHT_RULE,
            OtherSize383 => &Self::OTHER_SIZE383_RIGHT_RULE,
            OtherSize384 => &Self::OTHER_SIZE384_RIGHT_RULE,
            OtherSize385 => &Self::OTHER_SIZE385_RIGHT_RULE,
            OtherSize386 => &Self::OTHER_SIZE386_RIGHT_RULE,
            OtherSize387 => &Self::OTHER_SIZE387_RIGHT_RULE,
            OtherSize388 => &Self::OTHER_SIZE388_RIGHT_RULE,
            OtherSize389 => &Self::OTHER_SIZE389_RIGHT_RULE,
            OtherSize390 => &Self::OTHER_SIZE390_RIGHT_RULE,
            OtherSize391 => &Self::OTHER_SIZE391_RIGHT_RULE,
            OtherSize392 => &Self::OTHER_SIZE392_RIGHT_RULE,
            OtherSize393 => &Self::OTHER_SIZE393_RIGHT_RULE,
            OtherSize394 => &Self::OTHER_SIZE394_RIGHT_RULE,
            OtherSize395 => &Self::OTHER_SIZE395_RIGHT_RULE,
            OtherSize396 => &Self::OTHER_SIZE396_RIGHT_RULE,
            OtherSize397 => &Self::OTHER_SIZE397_RIGHT_RULE,
            OtherSize398 => &Self::OTHER_SIZE398_RIGHT_RULE,
            OtherSize399 => &Self::OTHER_SIZE399_RIGHT_RULE,
            OtherSize400 => &Self::OTHER_SIZE400_RIGHT_RULE,
            OtherSize401 => &Self::OTHER_SIZE401_RIGHT_RULE,
            OtherSize402 => &Self::OTHER_SIZE402_RIGHT_RULE,
            OtherSize403 => &Self::OTHER_SIZE403_RIGHT_RULE,
            OtherSize404 => &Self::OTHER_SIZE404_RIGHT_RULE,
            OtherSize405 => &Self::OTHER_SIZE405_RIGHT_RULE,
            OtherSize406 => &Self::OTHER_SIZE406_RIGHT_RULE,
            OtherSize407 => &Self::OTHER_SIZE407_RIGHT_RULE,
            OtherSize408 => &Self::OTHER_SIZE408_RIGHT_RULE,
            OtherSize409 => &Self::OTHER_SIZE409_RIGHT_RULE,
            OtherSize410 => &Self::OTHER_SIZE410_RIGHT_RULE,
            OtherSize411 => &Self::OTHER_SIZE411_RIGHT_RULE,
            OtherSize412 => &Self::OTHER_SIZE412_RIGHT_RULE,
            OtherSize413 => &Self::OTHER_SIZE413_RIGHT_RULE,
            OtherSize414 => &Self::OTHER_SIZE414_RIGHT_RULE,
            OtherSize415 => &Self::OTHER_SIZE415_RIGHT_RULE,
            OtherSize416 => &Self::OTHER_SIZE416_RIGHT_RULE,
            OtherSize417 => &Self::OTHER_SIZE417_RIGHT_RULE,
            OtherSize418 => &Self::OTHER_SIZE418_RIGHT_RULE,
            OtherSize419 => &Self::OTHER_SIZE419_RIGHT_RULE,
            OtherSize420 => &Self::OTHER_SIZE420_RIGHT_RULE,
            OtherSize421 => &Self::OTHER_SIZE421_RIGHT_RULE,
            OtherSize422 => &Self::OTHER_SIZE422_RIGHT_RULE,
            OtherSize423 => &Self::OTHER_SIZE423_RIGHT_RULE,
            OtherSize424 => &Self::OTHER_SIZE424_RIGHT_RULE,
            OtherSize425 => &Self::OTHER_SIZE425_RIGHT_RULE,
            OtherSize426 => &Self::OTHER_SIZE426_RIGHT_RULE,
            OtherSize427 => &Self::OTHER_SIZE427_RIGHT_RULE,
            OtherSize428 => &Self::OTHER_SIZE428_RIGHT_RULE,
            OtherSize429 => &Self::OTHER_SIZE429_RIGHT_RULE,
            OtherSize430 => &Self::OTHER_SIZE430_RIGHT_RULE,
            OtherSize431 => &Self::OTHER_SIZE431_RIGHT_RULE,
            OtherSize432 => &Self::OTHER_SIZE432_RIGHT_RULE,
            OtherSize433 => &Self::OTHER_SIZE433_RIGHT_RULE,
            OtherSize434 => &Self::OTHER_SIZE434_RIGHT_RULE,
            OtherSize435 => &Self::OTHER_SIZE435_RIGHT_RULE,
            OtherSize436 => &Self::OTHER_SIZE436_RIGHT_RULE,
            OtherSize437 => &Self::OTHER_SIZE437_RIGHT_RULE,
            OtherSize438 => &Self::OTHER_SIZE438_RIGHT_RULE,
            OtherSize439 => &Self::OTHER_SIZE439_RIGHT_RULE,
            OtherSize440 => &Self::OTHER_SIZE440_RIGHT_RULE,
            OtherSize441 => &Self::OTHER_SIZE441_RIGHT_RULE,
            OtherSize442 => &Self::OTHER_SIZE442_RIGHT_RULE,
            OtherSize443 => &Self::OTHER_SIZE443_RIGHT_RULE,
            OtherSize444 => &Self::OTHER_SIZE444_RIGHT_RULE,
            OtherSize445 => &Self::OTHER_SIZE445_RIGHT_RULE,
            OtherSize446 => &Self::OTHER_SIZE446_RIGHT_RULE,
            OtherSize447 => &Self::OTHER_SIZE447_RIGHT_RULE,
            OtherSize448 => &Self::OTHER_SIZE448_RIGHT_RULE,
            OtherSize449 => &Self::OTHER_SIZE449_RIGHT_RULE,
            OtherSize450 => &Self::OTHER_SIZE450_RIGHT_RULE,
            OtherSize451 => &Self::OTHER_SIZE451_RIGHT_RULE,
            OtherSize452 => &Self::OTHER_SIZE452_RIGHT_RULE,
            OtherSize453 => &Self::OTHER_SIZE453_RIGHT_RULE,
            OtherSize454 => &Self::OTHER_SIZE454_RIGHT_RULE,
            OtherSize455 => &Self::OTHER_SIZE455_RIGHT_RULE,
            OtherSize456 => &Self::OTHER_SIZE456_RIGHT_RULE,
            OtherSize457 => &Self::OTHER_SIZE457_RIGHT_RULE,
            OtherSize458 => &Self::OTHER_SIZE458_RIGHT_RULE,
            OtherSize459 => &Self::OTHER_SIZE459_RIGHT_RULE,
            OtherSize460 => &Self::OTHER_SIZE460_RIGHT_RULE,
            OtherSize461 => &Self::OTHER_SIZE461_RIGHT_RULE,
            OtherSize462 => &Self::OTHER_SIZE462_RIGHT_RULE,
            OtherSize463 => &Self::OTHER_SIZE463_RIGHT_RULE,
            OtherSize464 => &Self::OTHER_SIZE464_RIGHT_RULE,
            OtherSize465 => &Self::OTHER_SIZE465_RIGHT_RULE,
            OtherSize466 => &Self::OTHER_SIZE466_RIGHT_RULE,
            OtherSize467 => &Self::OTHER_SIZE467_RIGHT_RULE,
            OtherSize468 => &Self::OTHER_SIZE468_RIGHT_RULE,
            OtherSize469 => &Self::OTHER_SIZE469_RIGHT_RULE,
            OtherSize470 => &Self::OTHER_SIZE470_RIGHT_RULE,
            OtherSize471 => &Self::OTHER_SIZE471_RIGHT_RULE,
            OtherSize472 => &Self::OTHER_SIZE472_RIGHT_RULE,
            OtherSize473 => &Self::OTHER_SIZE473_RIGHT_RULE,
            OtherSize474 => &Self::OTHER_SIZE474_RIGHT_RULE,
            OtherSize475 => &Self::OTHER_SIZE475_RIGHT_RULE,
            OtherSize476 => &Self::OTHER_SIZE476_RIGHT_RULE,
            OtherSize477 => &Self::OTHER_SIZE477_RIGHT_RULE,
            OtherSize478 => &Self::OTHER_SIZE478_RIGHT_RULE,
            OtherSize479 => &Self::OTHER_SIZE479_RIGHT_RULE,
            OtherSize480 => &Self::OTHER_SIZE480_RIGHT_RULE,
            OtherSize481 => &Self::OTHER_SIZE481_RIGHT_RULE,
            OtherSize482 => &Self::OTHER_SIZE482_RIGHT_RULE,
            OtherSize483 => &Self::OTHER_SIZE483_RIGHT_RULE,
            OtherSize484 => &Self::OTHER_SIZE484_RIGHT_RULE,
            OtherSize485 => &Self::OTHER_SIZE485_RIGHT_RULE,
            OtherSize486 => &Self::OTHER_SIZE486_RIGHT_RULE,
            OtherSize487 => &Self::OTHER_SIZE487_RIGHT_RULE,
            OtherSize488 => &Self::OTHER_SIZE488_RIGHT_RULE,
            OtherSize489 => &Self::OTHER_SIZE489_RIGHT_RULE,
            OtherSize490 => &Self::OTHER_SIZE490_RIGHT_RULE,
            OtherSize491 => &Self::OTHER_SIZE491_RIGHT_RULE,
            OtherSize492 => &Self::OTHER_SIZE492_RIGHT_RULE,
            OtherSize493 => &Self::OTHER_SIZE493_RIGHT_RULE,
            OtherSize494 => &Self::OTHER_SIZE494_RIGHT_RULE,
            OtherSize495 => &Self::OTHER_SIZE495_RIGHT_RULE,
            OtherSize496 => &Self::OTHER_SIZE496_RIGHT_RULE,
            OtherSize497 => &Self::OTHER_SIZE497_RIGHT_RULE,
            OtherSize498 => &Self::OTHER_SIZE498_RIGHT_RULE,
            OtherSize499 => &Self::OTHER_SIZE499_RIGHT_RULE,
            OtherSize500 => &Self::OTHER_SIZE500_RIGHT_RULE,
            OtherSize501 => &Self::OTHER_SIZE501_RIGHT_RULE,
            OtherSize502 => &Self::OTHER_SIZE502_RIGHT_RULE,
            OtherSize503 => &Self::OTHER_SIZE503_RIGHT_RULE,
            OtherSize504 => &Self::OTHER_SIZE504_RIGHT_RULE,
            OtherSize505 => &Self::OTHER_SIZE505_RIGHT_RULE,
            OtherSize506 => &Self::OTHER_SIZE506_RIGHT_RULE,
            OtherSize507 => &Self::OTHER_SIZE507_RIGHT_RULE,
            OtherSize508 => &Self::OTHER_SIZE508_RIGHT_RULE,
            OtherSize509 => &Self::OTHER_SIZE509_RIGHT_RULE,
            OtherSize510 => &Self::OTHER_SIZE510_RIGHT_RULE,
            OtherSize511 => &Self::OTHER_SIZE511_RIGHT_RULE,
            OtherSize512 => &Self::OTHER_SIZE512_RIGHT_RULE,
            OtherSize513 => &Self::OTHER_SIZE513_RIGHT_RULE,
            OtherSize514 => &Self::OTHER_SIZE514_RIGHT_RULE,
            OtherSize515 => &Self::OTHER_SIZE515_RIGHT_RULE,
            OtherSize516 => &Self::OTHER_SIZE516_RIGHT_RULE,
            OtherSize517 => &Self::OTHER_SIZE517_RIGHT_RULE,
            OtherSize518 => &Self::OTHER_SIZE518_RIGHT_RULE,
            OtherSize519 => &Self::OTHER_SIZE519_RIGHT_RULE,
            OtherSize520 => &Self::OTHER_SIZE520_RIGHT_RULE,
            OtherSize521 => &Self::OTHER_SIZE521_RIGHT_RULE,
            OtherSize522 => &Self::OTHER_SIZE522_RIGHT_RULE,
            OtherSize523 => &Self::OTHER_SIZE523_RIGHT_RULE,
            OtherSize524 => &Self::OTHER_SIZE524_RIGHT_RULE,
            OtherSize525 => &Self::OTHER_SIZE525_RIGHT_RULE,
            OtherSize526 => &Self::OTHER_SIZE526_RIGHT_RULE,
            OtherSize527 => &Self::OTHER_SIZE527_RIGHT_RULE,
            OtherSize528 => &Self::OTHER_SIZE528_RIGHT_RULE,
            OtherSize529 => &Self::OTHER_SIZE529_RIGHT_RULE,
            OtherSize530 => &Self::OTHER_SIZE530_RIGHT_RULE,
            OtherSize531 => &Self::OTHER_SIZE531_RIGHT_RULE,
            OtherSize532 => &Self::OTHER_SIZE532_RIGHT_RULE,
            OtherSize533 => &Self::OTHER_SIZE533_RIGHT_RULE,
            OtherSize534 => &Self::OTHER_SIZE534_RIGHT_RULE,
            OtherSize535 => &Self::OTHER_SIZE535_RIGHT_RULE,
            OtherSize536 => &Self::OTHER_SIZE536_RIGHT_RULE,
            OtherSize537 => &Self::OTHER_SIZE537_RIGHT_RULE,
            OtherSize538 => &Self::OTHER_SIZE538_RIGHT_RULE,
            OtherSize539 => &Self::OTHER_SIZE539_RIGHT_RULE,
            OtherSize540 => &Self::OTHER_SIZE540_RIGHT_RULE,
            OtherSize541 => &Self::OTHER_SIZE541_RIGHT_RULE,
            OtherSize542 => &Self::OTHER_SIZE542_RIGHT_RULE,
            OtherSize543 => &Self::OTHER_SIZE543_RIGHT_RULE,
            OtherSize544 => &Self::OTHER_SIZE544_RIGHT_RULE,
            OtherSize545 => &Self::OTHER_SIZE545_RIGHT_RULE,
            OtherSize546 => &Self::OTHER_SIZE546_RIGHT_RULE,
            OtherSize547 => &Self::OTHER_SIZE547_RIGHT_RULE,
            OtherSize548 => &Self::OTHER_SIZE548_RIGHT_RULE,
            OtherSize549 => &Self::OTHER_SIZE549_RIGHT_RULE,
            OtherSize550 => &Self::OTHER_SIZE550_RIGHT_RULE,
            OtherSize551 => &Self::OTHER_SIZE551_RIGHT_RULE,
            OtherSize552 => &Self::OTHER_SIZE552_RIGHT_RULE,
            OtherSize553 => &Self::OTHER_SIZE553_RIGHT_RULE,
            OtherSize554 => &Self::OTHER_SIZE554_RIGHT_RULE,
            OtherSize555 => &Self::OTHER_SIZE555_RIGHT_RULE,
            OtherSize556 => &Self::OTHER_SIZE556_RIGHT_RULE,
            OtherSize557 => &Self::OTHER_SIZE557_RIGHT_RULE,
            OtherSize558 => &Self::OTHER_SIZE558_RIGHT_RULE,
            OtherSize559 => &Self::OTHER_SIZE559_RIGHT_RULE,
            OtherSize560 => &Self::OTHER_SIZE560_RIGHT_RULE,
            OtherSize561 => &Self::OTHER_SIZE561_RIGHT_RULE,
            OtherSize562 => &Self::OTHER_SIZE562_RIGHT_RULE,
            OtherSize563 => &Self::OTHER_SIZE563_RIGHT_RULE,
            OtherSize564 => &Self::OTHER_SIZE564_RIGHT_RULE,
            OtherSize565 => &Self::OTHER_SIZE565_RIGHT_RULE,
            OtherSize566 => &Self::OTHER_SIZE566_RIGHT_RULE,
            OtherSize567 => &Self::OTHER_SIZE567_RIGHT_RULE,
            OtherSize568 => &Self::OTHER_SIZE568_RIGHT_RULE,
            OtherSize569 => &Self::OTHER_SIZE569_RIGHT_RULE,
            OtherSize570 => &Self::OTHER_SIZE570_RIGHT_RULE,
            OtherSize571 => &Self::OTHER_SIZE571_RIGHT_RULE,
            OtherSize572 => &Self::OTHER_SIZE572_RIGHT_RULE,
            OtherSize573 => &Self::OTHER_SIZE573_RIGHT_RULE,
            OtherSize574 => &Self::OTHER_SIZE574_RIGHT_RULE,
            OtherSize575 => &Self::OTHER_SIZE575_RIGHT_RULE,
            OtherSize576 => &Self::OTHER_SIZE576_RIGHT_RULE,
            OtherSize577 => &Self::OTHER_SIZE577_RIGHT_RULE,
            OtherSize578 => &Self::OTHER_SIZE578_RIGHT_RULE,
            OtherSize579 => &Self::OTHER_SIZE579_RIGHT_RULE,
            OtherSize580 => &Self::OTHER_SIZE580_RIGHT_RULE,
            OtherSize581 => &Self::OTHER_SIZE581_RIGHT_RULE,
            OtherSize582 => &Self::OTHER_SIZE582_RIGHT_RULE,
            OtherSize583 => &Self::OTHER_SIZE583_RIGHT_RULE,
            OtherSize584 => &Self::OTHER_SIZE584_RIGHT_RULE,
            OtherSize585 => &Self::OTHER_SIZE585_RIGHT_RULE,
            OtherSize586 => &Self::OTHER_SIZE586_RIGHT_RULE,
            OtherSize587 => &Self::OTHER_SIZE587_RIGHT_RULE,
            OtherSize588 => &Self::OTHER_SIZE588_RIGHT_RULE,
            OtherSize589 => &Self::OTHER_SIZE589_RIGHT_RULE,
            OtherSize590 => &Self::OTHER_SIZE590_RIGHT_RULE,
            OtherSize591 => &Self::OTHER_SIZE591_RIGHT_RULE,
            OtherSize592 => &Self::OTHER_SIZE592_RIGHT_RULE,
            OtherSize593 => &Self::OTHER_SIZE593_RIGHT_RULE,
            OtherSize594 => &Self::OTHER_SIZE594_RIGHT_RULE,
            OtherSize595 => &Self::OTHER_SIZE595_RIGHT_RULE,
            OtherSize596 => &Self::OTHER_SIZE596_RIGHT_RULE,
            OtherSize597 => &Self::OTHER_SIZE597_RIGHT_RULE,
            OtherSize598 => &Self::OTHER_SIZE598_RIGHT_RULE,
            OtherSize599 => &Self::OTHER_SIZE599_RIGHT_RULE,
            OtherSize600 => &Self::OTHER_SIZE600_RIGHT_RULE,
            OtherSize601 => &Self::OTHER_SIZE601_RIGHT_RULE,
            OtherSize602 => &Self::OTHER_SIZE602_RIGHT_RULE,
            OtherSize603 => &Self::OTHER_SIZE603_RIGHT_RULE,
            OtherSize604 => &Self::OTHER_SIZE604_RIGHT_RULE,
            OtherSize605 => &Self::OTHER_SIZE605_RIGHT_RULE,
            OtherSize606 => &Self::OTHER_SIZE606_RIGHT_RULE,
            OtherSize607 => &Self::OTHER_SIZE607_RIGHT_RULE,
            OtherSize608 => &Self::OTHER_SIZE608_RIGHT_RULE,
            OtherSize609 => &Self::OTHER_SIZE609_RIGHT_RULE,
            OtherSize610 => &Self::OTHER_SIZE610_RIGHT_RULE,
            OtherSize611 => &Self::OTHER_SIZE611_RIGHT_RULE,
            OtherSize612 => &Self::OTHER_SIZE612_RIGHT_RULE,
            OtherSize613 => &Self::OTHER_SIZE613_RIGHT_RULE,
            OtherSize614 => &Self::OTHER_SIZE614_RIGHT_RULE,
            OtherSize615 => &Self::OTHER_SIZE615_RIGHT_RULE,
            OtherSize616 => &Self::OTHER_SIZE616_RIGHT_RULE,
            OtherSize617 => &Self::OTHER_SIZE617_RIGHT_RULE,
            OtherSize618 => &Self::OTHER_SIZE618_RIGHT_RULE,
            OtherSize619 => &Self::OTHER_SIZE619_RIGHT_RULE,
            OtherSize620 => &Self::OTHER_SIZE620_RIGHT_RULE,
            OtherSize621 => &Self::OTHER_SIZE621_RIGHT_RULE,
            OtherSize622 => &Self::OTHER_SIZE622_RIGHT_RULE,
            OtherSize623 => &Self::OTHER_SIZE623_RIGHT_RULE,
            OtherSize624 => &Self::OTHER_SIZE624_RIGHT_RULE,
            OtherSize625 => &Self::OTHER_SIZE625_RIGHT_RULE,
            OtherSize626 => &Self::OTHER_SIZE626_RIGHT_RULE,
            OtherSize627 => &Self::OTHER_SIZE627_RIGHT_RULE,
            OtherSize628 => &Self::OTHER_SIZE628_RIGHT_RULE,
            OtherSize629 => &Self::OTHER_SIZE629_RIGHT_RULE,
            OtherSize630 => &Self::OTHER_SIZE630_RIGHT_RULE,
            OtherSize631 => &Self::OTHER_SIZE631_RIGHT_RULE,
            OtherSize632 => &Self::OTHER_SIZE632_RIGHT_RULE,
            OtherSize633 => &Self::OTHER_SIZE633_RIGHT_RULE,
            OtherSize634 => &Self::OTHER_SIZE634_RIGHT_RULE,
            OtherSize635 => &Self::OTHER_SIZE635_RIGHT_RULE,
            OtherSize636 => &Self::OTHER_SIZE636_RIGHT_RULE,
            OtherSize637 => &Self::OTHER_SIZE637_RIGHT_RULE,
            OtherSize638 => &Self::OTHER_SIZE638_RIGHT_RULE,
            OtherSize639 => &Self::OTHER_SIZE639_RIGHT_RULE,
            OtherSize640 => &Self::OTHER_SIZE640_RIGHT_RULE,
            OtherSize641 => &Self::OTHER_SIZE641_RIGHT_RULE,
            OtherSize642 => &Self::OTHER_SIZE642_RIGHT_RULE,
            OtherSize643 => &Self::OTHER_SIZE643_RIGHT_RULE,
            OtherSize644 => &Self::OTHER_SIZE644_RIGHT_RULE,
            OtherSize645 => &Self::OTHER_SIZE645_RIGHT_RULE,
            OtherSize646 => &Self::OTHER_SIZE646_RIGHT_RULE,
            OtherSize647 => &Self::OTHER_SIZE647_RIGHT_RULE,
            OtherSize648 => &Self::OTHER_SIZE648_RIGHT_RULE,
            OtherSize649 => &Self::OTHER_SIZE649_RIGHT_RULE,
            OtherSize650 => &Self::OTHER_SIZE650_RIGHT_RULE,
            OtherSize651 => &Self::OTHER_SIZE651_RIGHT_RULE,
            OtherSize652 => &Self::OTHER_SIZE652_RIGHT_RULE,
            OtherSize653 => &Self::OTHER_SIZE653_RIGHT_RULE,
            OtherSize654 => &Self::OTHER_SIZE654_RIGHT_RULE,
            OtherSize655 => &Self::OTHER_SIZE655_RIGHT_RULE,
            OtherSize656 => &Self::OTHER_SIZE656_RIGHT_RULE,
            OtherSize657 => &Self::OTHER_SIZE657_RIGHT_RULE,
            OtherSize658 => &Self::OTHER_SIZE658_RIGHT_RULE,
            OtherSize659 => &Self::OTHER_SIZE659_RIGHT_RULE,
            OtherSize660 => &Self::OTHER_SIZE660_RIGHT_RULE,
            OtherSize661 => &Self::OTHER_SIZE661_RIGHT_RULE,
            OtherSize662 => &Self::OTHER_SIZE662_RIGHT_RULE,
            OtherSize663 => &Self::OTHER_SIZE663_RIGHT_RULE,
            OtherSize664 => &Self::OTHER_SIZE664_RIGHT_RULE,
            OtherSize665 => &Self::OTHER_SIZE665_RIGHT_RULE,
            OtherSize666 => &Self::OTHER_SIZE666_RIGHT_RULE,
            OtherSize667 => &Self::OTHER_SIZE667_RIGHT_RULE,
            OtherSize668 => &Self::OTHER_SIZE668_RIGHT_RULE,
            OtherSize669 => &Self::OTHER_SIZE669_RIGHT_RULE,
            OtherSize670 => &Self::OTHER_SIZE670_RIGHT_RULE,
            OtherSize671 => &Self::OTHER_SIZE671_RIGHT_RULE,
            OtherSize672 => &Self::OTHER_SIZE672_RIGHT_RULE,
            OtherSize673 => &Self::OTHER_SIZE673_RIGHT_RULE,
            OtherSize674 => &Self::OTHER_SIZE674_RIGHT_RULE,
            OtherSize675 => &Self::OTHER_SIZE675_RIGHT_RULE,
            OtherSize676 => &Self::OTHER_SIZE676_RIGHT_RULE,
            OtherSize677 => &Self::OTHER_SIZE677_RIGHT_RULE,
            OtherSize678 => &Self::OTHER_SIZE678_RIGHT_RULE,
            OtherSize679 => &Self::OTHER_SIZE679_RIGHT_RULE,
            OtherSize680 => &Self::OTHER_SIZE680_RIGHT_RULE,
            OtherSize681 => &Self::OTHER_SIZE681_RIGHT_RULE,
            OtherSize682 => &Self::OTHER_SIZE682_RIGHT_RULE,
            OtherSize683 => &Self::OTHER_SIZE683_RIGHT_RULE,
            OtherSize684 => &Self::OTHER_SIZE684_RIGHT_RULE,
            OtherSize685 => &Self::OTHER_SIZE685_RIGHT_RULE,
            OtherSize686 => &Self::OTHER_SIZE686_RIGHT_RULE,
            OtherSize687 => &Self::OTHER_SIZE687_RIGHT_RULE,
            OtherSize688 => &Self::OTHER_SIZE688_RIGHT_RULE,
            OtherSize689 => &Self::OTHER_SIZE689_RIGHT_RULE,
            OtherSize690 => &Self::OTHER_SIZE690_RIGHT_RULE,
            OtherSize691 => &Self::OTHER_SIZE691_RIGHT_RULE,
            OtherSize692 => &Self::OTHER_SIZE692_RIGHT_RULE,
            OtherSize693 => &Self::OTHER_SIZE693_RIGHT_RULE,
            OtherSize694 => &Self::OTHER_SIZE694_RIGHT_RULE,
            OtherSize695 => &Self::OTHER_SIZE695_RIGHT_RULE,
            OtherSize696 => &Self::OTHER_SIZE696_RIGHT_RULE,
            OtherSize697 => &Self::OTHER_SIZE697_RIGHT_RULE,
            OtherSize698 => &Self::OTHER_SIZE698_RIGHT_RULE,
            OtherSize699 => &Self::OTHER_SIZE699_RIGHT_RULE,
            OtherSize700 => &Self::OTHER_SIZE700_RIGHT_RULE,
            OtherSize701 => &Self::OTHER_SIZE701_RIGHT_RULE,
            OtherSize702 => &Self::OTHER_SIZE702_RIGHT_RULE,
            OtherSize703 => &Self::OTHER_SIZE703_RIGHT_RULE,
            OtherSize704 => &Self::OTHER_SIZE704_RIGHT_RULE,
            OtherSize705 => &Self::OTHER_SIZE705_RIGHT_RULE,
            OtherSize706 => &Self::OTHER_SIZE706_RIGHT_RULE,
            OtherSize707 => &Self::OTHER_SIZE707_RIGHT_RULE,
            OtherSize708 => &Self::OTHER_SIZE708_RIGHT_RULE,
            OtherSize709 => &Self::OTHER_SIZE709_RIGHT_RULE,
            OtherSize710 => &Self::OTHER_SIZE710_RIGHT_RULE,
            OtherSize711 => &Self::OTHER_SIZE711_RIGHT_RULE,
            OtherSize712 => &Self::OTHER_SIZE712_RIGHT_RULE,
            OtherSize713 => &Self::OTHER_SIZE713_RIGHT_RULE,
            OtherSize714 => &Self::OTHER_SIZE714_RIGHT_RULE,
            OtherSize715 => &Self::OTHER_SIZE715_RIGHT_RULE,
            OtherSize716 => &Self::OTHER_SIZE716_RIGHT_RULE,
            OtherSize717 => &Self::OTHER_SIZE717_RIGHT_RULE,
            OtherSize718 => &Self::OTHER_SIZE718_RIGHT_RULE,
            OtherSize719 => &Self::OTHER_SIZE719_RIGHT_RULE,
            OtherSize720 => &Self::OTHER_SIZE720_RIGHT_RULE,
            OtherSize721 => &Self::OTHER_SIZE721_RIGHT_RULE,
            OtherSize722 => &Self::OTHER_SIZE722_RIGHT_RULE,
            OtherSize723 => &Self::OTHER_SIZE723_RIGHT_RULE,
            OtherSize724 => &Self::OTHER_SIZE724_RIGHT_RULE,
            OtherSize725 => &Self::OTHER_SIZE725_RIGHT_RULE,
            OtherSize726 => &Self::OTHER_SIZE726_RIGHT_RULE,
            OtherSize727 => &Self::OTHER_SIZE727_RIGHT_RULE,
            OtherSize728 => &Self::OTHER_SIZE728_RIGHT_RULE,
            OtherSize729 => &Self::OTHER_SIZE729_RIGHT_RULE,
            OtherSize730 => &Self::OTHER_SIZE730_RIGHT_RULE,
            OtherSize731 => &Self::OTHER_SIZE731_RIGHT_RULE,
            OtherSize732 => &Self::OTHER_SIZE732_RIGHT_RULE,
            OtherSize733 => &Self::OTHER_SIZE733_RIGHT_RULE,
            OtherSize734 => &Self::OTHER_SIZE734_RIGHT_RULE,
            OtherSize735 => &Self::OTHER_SIZE735_RIGHT_RULE,
            OtherSize736 => &Self::OTHER_SIZE736_RIGHT_RULE,
            OtherSize737 => &Self::OTHER_SIZE737_RIGHT_RULE,
            OtherSize738 => &Self::OTHER_SIZE738_RIGHT_RULE,
            OtherSize739 => &Self::OTHER_SIZE739_RIGHT_RULE,
            OtherSize740 => &Self::OTHER_SIZE740_RIGHT_RULE,
            OtherSize741 => &Self::OTHER_SIZE741_RIGHT_RULE,
            OtherSize742 => &Self::OTHER_SIZE742_RIGHT_RULE,
            OtherSize743 => &Self::OTHER_SIZE743_RIGHT_RULE,
            OtherSize744 => &Self::OTHER_SIZE744_RIGHT_RULE,
            OtherSize745 => &Self::OTHER_SIZE745_RIGHT_RULE,
            OtherSize746 => &Self::OTHER_SIZE746_RIGHT_RULE,
            OtherSize747 => &Self::OTHER_SIZE747_RIGHT_RULE,
            OtherSize748 => &Self::OTHER_SIZE748_RIGHT_RULE,
            OtherSize749 => &Self::OTHER_SIZE749_RIGHT_RULE,
            OtherSize750 => &Self::OTHER_SIZE750_RIGHT_RULE,
            OtherSize751 => &Self::OTHER_SIZE751_RIGHT_RULE,
            OtherSize752 => &Self::OTHER_SIZE752_RIGHT_RULE,
            OtherSize753 => &Self::OTHER_SIZE753_RIGHT_RULE,
            OtherSize754 => &Self::OTHER_SIZE754_RIGHT_RULE,
            OtherSize755 => &Self::OTHER_SIZE755_RIGHT_RULE,
            OtherSize756 => &Self::OTHER_SIZE756_RIGHT_RULE,
            OtherSize757 => &Self::OTHER_SIZE757_RIGHT_RULE,
            OtherSize758 => &Self::OTHER_SIZE758_RIGHT_RULE,
            OtherSize759 => &Self::OTHER_SIZE759_RIGHT_RULE,
            OtherSize760 => &Self::OTHER_SIZE760_RIGHT_RULE,
            OtherSize761 => &Self::OTHER_SIZE761_RIGHT_RULE,
            OtherSize762 => &Self::OTHER_SIZE762_RIGHT_RULE,
            OtherSize763 => &Self::OTHER_SIZE763_RIGHT_RULE,
            OtherSize764 => &Self::OTHER_SIZE764_RIGHT_RULE,
            OtherSize765 => &Self::OTHER_SIZE765_RIGHT_RULE,
            OtherSize766 => &Self::OTHER_SIZE766_RIGHT_RULE,
            OtherSize767 => &Self::OTHER_SIZE767_RIGHT_RULE,
            OtherSize768 => &Self::OTHER_SIZE768_RIGHT_RULE,
            OtherSize769 => &Self::OTHER_SIZE769_RIGHT_RULE,
            OtherSize770 => &Self::OTHER_SIZE770_RIGHT_RULE,
            OtherSize771 => &Self::OTHER_SIZE771_RIGHT_RULE,
            OtherSize772 => &Self::OTHER_SIZE772_RIGHT_RULE,
            OtherSize773 => &Self::OTHER_SIZE773_RIGHT_RULE,
            OtherSize774 => &Self::OTHER_SIZE774_RIGHT_RULE,
            OtherSize775 => &Self::OTHER_SIZE775_RIGHT_RULE,
            OtherSize776 => &Self::OTHER_SIZE776_RIGHT_RULE,
            OtherSize777 => &Self::OTHER_SIZE777_RIGHT_RULE,
            OtherSize778 => &Self::OTHER_SIZE778_RIGHT_RULE,
            OtherSize779 => &Self::OTHER_SIZE779_RIGHT_RULE,
            OtherSize780 => &Self::OTHER_SIZE780_RIGHT_RULE,
            OtherSize781 => &Self::OTHER_SIZE781_RIGHT_RULE,
            OtherSize782 => &Self::OTHER_SIZE782_RIGHT_RULE,
            OtherSize783 => &Self::OTHER_SIZE783_RIGHT_RULE,
            OtherSize784 => &Self::OTHER_SIZE784_RIGHT_RULE,
            OtherSize785 => &Self::OTHER_SIZE785_RIGHT_RULE,
            OtherSize786 => &Self::OTHER_SIZE786_RIGHT_RULE,
            OtherSize787 => &Self::OTHER_SIZE787_RIGHT_RULE,
            OtherSize788 => &Self::OTHER_SIZE788_RIGHT_RULE,
            OtherSize789 => &Self::OTHER_SIZE789_RIGHT_RULE,
            OtherSize790 => &Self::OTHER_SIZE790_RIGHT_RULE,
            OtherSize791 => &Self::OTHER_SIZE791_RIGHT_RULE,
            OtherSize792 => &Self::OTHER_SIZE792_RIGHT_RULE,
            OtherSize793 => &Self::OTHER_SIZE793_RIGHT_RULE,
            OtherSize794 => &Self::OTHER_SIZE794_RIGHT_RULE,
            OtherSize795 => &Self::OTHER_SIZE795_RIGHT_RULE,
            OtherSize796 => &Self::OTHER_SIZE796_RIGHT_RULE,
            OtherSize797 => &Self::OTHER_SIZE797_RIGHT_RULE,
            OtherSize798 => &Self::OTHER_SIZE798_RIGHT_RULE,
            OtherSize799 => &Self::OTHER_SIZE799_RIGHT_RULE,
            OtherSize800 => &Self::OTHER_SIZE800_RIGHT_RULE,
            OtherSize801 => &Self::OTHER_SIZE801_RIGHT_RULE,
            OtherSize802 => &Self::OTHER_SIZE802_RIGHT_RULE,
            OtherSize803 => &Self::OTHER_SIZE803_RIGHT_RULE,
            OtherSize804 => &Self::OTHER_SIZE804_RIGHT_RULE,
            OtherSize805 => &Self::OTHER_SIZE805_RIGHT_RULE,
            OtherSize806 => &Self::OTHER_SIZE806_RIGHT_RULE,
            OtherSize807 => &Self::OTHER_SIZE807_RIGHT_RULE,
            OtherSize808 => &Self::OTHER_SIZE808_RIGHT_RULE,
            OtherSize809 => &Self::OTHER_SIZE809_RIGHT_RULE,
            OtherSize810 => &Self::OTHER_SIZE810_RIGHT_RULE,
            OtherSize811 => &Self::OTHER_SIZE811_RIGHT_RULE,
            OtherSize812 => &Self::OTHER_SIZE812_RIGHT_RULE,
            OtherSize813 => &Self::OTHER_SIZE813_RIGHT_RULE,
            OtherSize814 => &Self::OTHER_SIZE814_RIGHT_RULE,
            OtherSize815 => &Self::OTHER_SIZE815_RIGHT_RULE,
            OtherSize816 => &Self::OTHER_SIZE816_RIGHT_RULE,
            OtherSize817 => &Self::OTHER_SIZE817_RIGHT_RULE,
            OtherSize818 => &Self::OTHER_SIZE818_RIGHT_RULE,
            OtherSize819 => &Self::OTHER_SIZE819_RIGHT_RULE,
            OtherSize820 => &Self::OTHER_SIZE820_RIGHT_RULE,
            OtherSize821 => &Self::OTHER_SIZE821_RIGHT_RULE,
            OtherSize822 => &Self::OTHER_SIZE822_RIGHT_RULE,
            OtherSize823 => &Self::OTHER_SIZE823_RIGHT_RULE,
            OtherSize824 => &Self::OTHER_SIZE824_RIGHT_RULE,
            OtherSize825 => &Self::OTHER_SIZE825_RIGHT_RULE,
            OtherSize826 => &Self::OTHER_SIZE826_RIGHT_RULE,
            OtherSize827 => &Self::OTHER_SIZE827_RIGHT_RULE,
            OtherSize828 => &Self::OTHER_SIZE828_RIGHT_RULE,
            OtherSize829 => &Self::OTHER_SIZE829_RIGHT_RULE,
            OtherSize830 => &Self::OTHER_SIZE830_RIGHT_RULE,
            OtherSize831 => &Self::OTHER_SIZE831_RIGHT_RULE,
            OtherSize832 => &Self::OTHER_SIZE832_RIGHT_RULE,
            OtherSize833 => &Self::OTHER_SIZE833_RIGHT_RULE,
            OtherSize834 => &Self::OTHER_SIZE834_RIGHT_RULE,
            OtherSize835 => &Self::OTHER_SIZE835_RIGHT_RULE,
            OtherSize836 => &Self::OTHER_SIZE836_RIGHT_RULE,
            OtherSize837 => &Self::OTHER_SIZE837_RIGHT_RULE,
            OtherSize838 => &Self::OTHER_SIZE838_RIGHT_RULE,
            OtherSize839 => &Self::OTHER_SIZE839_RIGHT_RULE,
            OtherSize840 => &Self::OTHER_SIZE840_RIGHT_RULE,
            OtherSize841 => &Self::OTHER_SIZE841_RIGHT_RULE,
            OtherSize842 => &Self::OTHER_SIZE842_RIGHT_RULE,
            OtherSize843 => &Self::OTHER_SIZE843_RIGHT_RULE,
            OtherSize844 => &Self::OTHER_SIZE844_RIGHT_RULE,
            OtherSize845 => &Self::OTHER_SIZE845_RIGHT_RULE,
            OtherSize846 => &Self::OTHER_SIZE846_RIGHT_RULE,
            OtherSize847 => &Self::OTHER_SIZE847_RIGHT_RULE,
            OtherSize848 => &Self::OTHER_SIZE848_RIGHT_RULE,
            OtherSize849 => &Self::OTHER_SIZE849_RIGHT_RULE,
            OtherSize850 => &Self::OTHER_SIZE850_RIGHT_RULE,
            OtherSize851 => &Self::OTHER_SIZE851_RIGHT_RULE,
            OtherSize852 => &Self::OTHER_SIZE852_RIGHT_RULE,
            OtherSize853 => &Self::OTHER_SIZE853_RIGHT_RULE,
            OtherSize854 => &Self::OTHER_SIZE854_RIGHT_RULE,
            OtherSize855 => &Self::OTHER_SIZE855_RIGHT_RULE,
            OtherSize856 => &Self::OTHER_SIZE856_RIGHT_RULE,
            OtherSize857 => &Self::OTHER_SIZE857_RIGHT_RULE,
            OtherSize858 => &Self::OTHER_SIZE858_RIGHT_RULE,
            OtherSize859 => &Self::OTHER_SIZE859_RIGHT_RULE,
            OtherSize860 => &Self::OTHER_SIZE860_RIGHT_RULE,
            OtherSize861 => &Self::OTHER_SIZE861_RIGHT_RULE,
            OtherSize862 => &Self::OTHER_SIZE862_RIGHT_RULE,
            OtherSize863 => &Self::OTHER_SIZE863_RIGHT_RULE,
            OtherSize864 => &Self::OTHER_SIZE864_RIGHT_RULE,
            OtherSize865 => &Self::OTHER_SIZE865_RIGHT_RULE,
            OtherSize866 => &Self::OTHER_SIZE866_RIGHT_RULE,
            OtherSize867 => &Self::OTHER_SIZE867_RIGHT_RULE,
            OtherSize868 => &Self::OTHER_SIZE868_RIGHT_RULE,
            OtherSize869 => &Self::OTHER_SIZE869_RIGHT_RULE,
            OtherSize870 => &Self::OTHER_SIZE870_RIGHT_RULE,
            OtherSize871 => &Self::OTHER_SIZE871_RIGHT_RULE,
            OtherSize872 => &Self::OTHER_SIZE872_RIGHT_RULE,
            OtherSize873 => &Self::OTHER_SIZE873_RIGHT_RULE,
            OtherSize874 => &Self::OTHER_SIZE874_RIGHT_RULE,
            OtherSize875 => &Self::OTHER_SIZE875_RIGHT_RULE,
            OtherSize876 => &Self::OTHER_SIZE876_RIGHT_RULE,
            OtherSize877 => &Self::OTHER_SIZE877_RIGHT_RULE,
            OtherSize878 => &Self::OTHER_SIZE878_RIGHT_RULE,
            OtherSize879 => &Self::OTHER_SIZE879_RIGHT_RULE,
            OtherSize880 => &Self::OTHER_SIZE880_RIGHT_RULE,
            OtherSize881 => &Self::OTHER_SIZE881_RIGHT_RULE,
            OtherSize882 => &Self::OTHER_SIZE882_RIGHT_RULE,
            OtherSize883 => &Self::OTHER_SIZE883_RIGHT_RULE,
            OtherSize884 => &Self::OTHER_SIZE884_RIGHT_RULE,
            OtherSize885 => &Self::OTHER_SIZE885_RIGHT_RULE,
            OtherSize886 => &Self::OTHER_SIZE886_RIGHT_RULE,
            OtherSize887 => &Self::OTHER_SIZE887_RIGHT_RULE,
            OtherSize888 => &Self::OTHER_SIZE888_RIGHT_RULE,
            OtherSize889 => &Self::OTHER_SIZE889_RIGHT_RULE,
            OtherSize890 => &Self::OTHER_SIZE890_RIGHT_RULE,
            OtherSize891 => &Self::OTHER_SIZE891_RIGHT_RULE,
            OtherSize892 => &Self::OTHER_SIZE892_RIGHT_RULE,
            OtherSize893 => &Self::OTHER_SIZE893_RIGHT_RULE,
            OtherSize894 => &Self::OTHER_SIZE894_RIGHT_RULE,
            OtherSize895 => &Self::OTHER_SIZE895_RIGHT_RULE,
            OtherSize896 => &Self::OTHER_SIZE896_RIGHT_RULE,
            OtherSize897 => &Self::OTHER_SIZE897_RIGHT_RULE,
            OtherSize898 => &Self::OTHER_SIZE898_RIGHT_RULE,
            OtherSize899 => &Self::OTHER_SIZE899_RIGHT_RULE,
            OtherSize900 => &Self::OTHER_SIZE900_RIGHT_RULE,
            OtherSize901 => &Self::OTHER_SIZE901_RIGHT_RULE,
            OtherSize902 => &Self::OTHER_SIZE902_RIGHT_RULE,
            OtherSize903 => &Self::OTHER_SIZE903_RIGHT_RULE,
            OtherSize904 => &Self::OTHER_SIZE904_RIGHT_RULE,
            OtherSize905 => &Self::OTHER_SIZE905_RIGHT_RULE,
            OtherSize906 => &Self::OTHER_SIZE906_RIGHT_RULE,
            OtherSize907 => &Self::OTHER_SIZE907_RIGHT_RULE,
            OtherSize908 => &Self::OTHER_SIZE908_RIGHT_RULE,
            OtherSize909 => &Self::OTHER_SIZE909_RIGHT_RULE,
            OtherSize910 => &Self::OTHER_SIZE910_RIGHT_RULE,
            OtherSize911 => &Self::OTHER_SIZE911_RIGHT_RULE,
            OtherSize912 => &Self::OTHER_SIZE912_RIGHT_RULE,
            OtherSize913 => &Self::OTHER_SIZE913_RIGHT_RULE,
            OtherSize914 => &Self::OTHER_SIZE914_RIGHT_RULE,
            OtherSize915 => &Self::OTHER_SIZE915_RIGHT_RULE,
            OtherSize916 => &Self::OTHER_SIZE916_RIGHT_RULE,
            OtherSize917 => &Self::OTHER_SIZE917_RIGHT_RULE,
            OtherSize918 => &Self::OTHER_SIZE918_RIGHT_RULE,
            OtherSize919 => &Self::OTHER_SIZE919_RIGHT_RULE,
            OtherSize920 => &Self::OTHER_SIZE920_RIGHT_RULE,
            OtherSize921 => &Self::OTHER_SIZE921_RIGHT_RULE,
            OtherSize922 => &Self::OTHER_SIZE922_RIGHT_RULE,
            OtherSize923 => &Self::OTHER_SIZE923_RIGHT_RULE,
            OtherSize924 => &Self::OTHER_SIZE924_RIGHT_RULE,
            OtherSize925 => &Self::OTHER_SIZE925_RIGHT_RULE,
            OtherSize926 => &Self::OTHER_SIZE926_RIGHT_RULE,
            OtherSize927 => &Self::OTHER_SIZE927_RIGHT_RULE,
            OtherSize928 => &Self::OTHER_SIZE928_RIGHT_RULE,
            OtherSize929 => &Self::OTHER_SIZE929_RIGHT_RULE,
            OtherSize930 => &Self::OTHER_SIZE930_RIGHT_RULE,
            OtherSize931 => &Self::OTHER_SIZE931_RIGHT_RULE,
            OtherSize932 => &Self::OTHER_SIZE932_RIGHT_RULE,
            OtherSize933 => &Self::OTHER_SIZE933_RIGHT_RULE,
            OtherSize934 => &Self::OTHER_SIZE934_RIGHT_RULE,
            OtherSize935 => &Self::OTHER_SIZE935_RIGHT_RULE,
            OtherSize936 => &Self::OTHER_SIZE936_RIGHT_RULE,
            OtherSize937 => &Self::OTHER_SIZE937_RIGHT_RULE,
            OtherSize938 => &Self::OTHER_SIZE938_RIGHT_RULE,
            OtherSize939 => &Self::OTHER_SIZE939_RIGHT_RULE,
            OtherSize940 => &Self::OTHER_SIZE940_RIGHT_RULE,
            OtherSize941 => &Self::OTHER_SIZE941_RIGHT_RULE,
            OtherSize942 => &Self::OTHER_SIZE942_RIGHT_RULE,
            OtherSize943 => &Self::OTHER_SIZE943_RIGHT_RULE,
            OtherSize944 => &Self::OTHER_SIZE944_RIGHT_RULE,
            OtherSize945 => &Self::OTHER_SIZE945_RIGHT_RULE,
            OtherSize946 => &Self::OTHER_SIZE946_RIGHT_RULE,
            OtherSize947 => &Self::OTHER_SIZE947_RIGHT_RULE,
            OtherSize948 => &Self::OTHER_SIZE948_RIGHT_RULE,
            OtherSize949 => &Self::OTHER_SIZE949_RIGHT_RULE,
            OtherSize950 => &Self::OTHER_SIZE950_RIGHT_RULE,
            OtherSize951 => &Self::OTHER_SIZE951_RIGHT_RULE,
            OtherSize952 => &Self::OTHER_SIZE952_RIGHT_RULE,
            OtherSize953 => &Self::OTHER_SIZE953_RIGHT_RULE,
            OtherSize954 => &Self::OTHER_SIZE954_RIGHT_RULE,
            OtherSize955 => &Self::OTHER_SIZE955_RIGHT_RULE,
            OtherSize956 => &Self::OTHER_SIZE956_RIGHT_RULE,
            OtherSize957 => &Self::OTHER_SIZE957_RIGHT_RULE,
            OtherSize958 => &Self::OTHER_SIZE958_RIGHT_RULE,
            OtherSize959 => &Self::OTHER_SIZE959_RIGHT_RULE,
            OtherSize960 => &Self::OTHER_SIZE960_RIGHT_RULE,
            OtherSize961 => &Self::OTHER_SIZE961_RIGHT_RULE,
            OtherSize962 => &Self::OTHER_SIZE962_RIGHT_RULE,
            OtherSize963 => &Self::OTHER_SIZE963_RIGHT_RULE,
            OtherSize964 => &Self::OTHER_SIZE964_RIGHT_RULE,
            OtherSize965 => &Self::OTHER_SIZE965_RIGHT_RULE,
            OtherSize966 => &Self::OTHER_SIZE966_RIGHT_RULE,
            OtherSize967 => &Self::OTHER_SIZE967_RIGHT_RULE,
            OtherSize968 => &Self::OTHER_SIZE968_RIGHT_RULE,
            OtherSize969 => &Self::OTHER_SIZE969_RIGHT_RULE,
            OtherSize970 => &Self::OTHER_SIZE970_RIGHT_RULE,
            OtherSize971 => &Self::OTHER_SIZE971_RIGHT_RULE,
            OtherSize972 => &Self::OTHER_SIZE972_RIGHT_RULE,
            OtherSize973 => &Self::OTHER_SIZE973_RIGHT_RULE,
            OtherSize974 => &Self::OTHER_SIZE974_RIGHT_RULE,
            OtherSize975 => &Self::OTHER_SIZE975_RIGHT_RULE,
            OtherSize976 => &Self::OTHER_SIZE976_RIGHT_RULE,
            OtherSize977 => &Self::OTHER_SIZE977_RIGHT_RULE,
            OtherSize978 => &Self::OTHER_SIZE978_RIGHT_RULE,
            OtherSize979 => &Self::OTHER_SIZE979_RIGHT_RULE,
            OtherSize980 => &Self::OTHER_SIZE980_RIGHT_RULE,
            OtherSize981 => &Self::OTHER_SIZE981_RIGHT_RULE,
            OtherSize982 => &Self::OTHER_SIZE982_RIGHT_RULE,
            OtherSize983 => &Self::OTHER_SIZE983_RIGHT_RULE,
            OtherSize984 => &Self::OTHER_SIZE984_RIGHT_RULE,
            OtherSize985 => &Self::OTHER_SIZE985_RIGHT_RULE,
            OtherSize986 => &Self::OTHER_SIZE986_RIGHT_RULE,
            OtherSize987 => &Self::OTHER_SIZE987_RIGHT_RULE,
            OtherSize988 => &Self::OTHER_SIZE988_RIGHT_RULE,
            OtherSize989 => &Self::OTHER_SIZE989_RIGHT_RULE,
            OtherSize990 => &Self::OTHER_SIZE990_RIGHT_RULE,
            OtherSize991 => &Self::OTHER_SIZE991_RIGHT_RULE,
            OtherSize992 => &Self::OTHER_SIZE992_RIGHT_RULE,
            OtherSize993 => &Self::OTHER_SIZE993_RIGHT_RULE,
            OtherSize994 => &Self::OTHER_SIZE994_RIGHT_RULE,
            OtherSize995 => &Self::OTHER_SIZE995_RIGHT_RULE,
            OtherSize996 => &Self::OTHER_SIZE996_RIGHT_RULE,
            OtherSize997 => &Self::OTHER_SIZE997_RIGHT_RULE,
            OtherSize998 => &Self::OTHER_SIZE998_RIGHT_RULE,
            OtherSize999 => &Self::OTHER_SIZE999_RIGHT_RULE,
            OtherSize1000 => &Self::OTHER_SIZE1000_RIGHT_RULE,
            OtherSize1001 => &Self::OTHER_SIZE1001_RIGHT_RULE,
            OtherSize1002 => &Self::OTHER_SIZE1002_RIGHT_RULE,
            OtherSize1003 => &Self::OTHER_SIZE1003_RIGHT_RULE,
            OtherSize1004 => &Self::OTHER_SIZE1004_RIGHT_RULE,
            OtherSize1005 => &Self::OTHER_SIZE1005_RIGHT_RULE,
            OtherSize1006 => &Self::OTHER_SIZE1006_RIGHT_RULE,
            OtherSize1007 => &Self::OTHER_SIZE1007_RIGHT_RULE,
            OtherSize1008 => &Self::OTHER_SIZE1008_RIGHT_RULE,
            OtherSize1009 => &Self::OTHER_SIZE1009_RIGHT_RULE,
            OtherSize1010 => &Self::OTHER_SIZE1010_RIGHT_RULE,
            OtherSize1011 => &Self::OTHER_SIZE1011_RIGHT_RULE,
            OtherSize1012 => &Self::OTHER_SIZE1012_RIGHT_RULE,
            OtherSize1013 => &Self::OTHER_SIZE1013_RIGHT_RULE,
            OtherSize1014 => &Self::OTHER_SIZE1014_RIGHT_RULE,
            OtherSize1015 => &Self::OTHER_SIZE1015_RIGHT_RULE,
            OtherSize1016 => &Self::OTHER_SIZE1016_RIGHT_RULE,
            OtherSize1017 => &Self::OTHER_SIZE1017_RIGHT_RULE,
            OtherSize1018 => &Self::OTHER_SIZE1018_RIGHT_RULE,
            OtherSize1019 => &Self::OTHER_SIZE1019_RIGHT_RULE,
            OtherSize1020 => &Self::OTHER_SIZE1020_RIGHT_RULE,
            OtherSize1021 => &Self::OTHER_SIZE1021_RIGHT_RULE,
            OtherSize1022 => &Self::OTHER_SIZE1022_RIGHT_RULE,
            OtherSize1023 => &Self::OTHER_SIZE1023_RIGHT_RULE,
            OtherSize1024 => &Self::OTHER_SIZE1024_RIGHT_RULE,

            // Data Chunk
            Data => &Self::DATA_RIGHT_RULE,
            DataSize => &Self::DATA_SIZE_RIGHT_RULE,

            U16 => &Self::U16_RIGHT_RULE,
            U32 => &Self::U32_RIGHT_RULE,
            U128 => &Self::U128_RIGHT_RULE,
        })
    }
}
