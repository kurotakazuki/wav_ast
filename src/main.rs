use crate::output::WavOutput;
use crate::variable::WavVariable;
use mpl::parse::Parse;
use mpl::rules::{RightRule, RightRuleKind};
use mpl::span::StartAndLenSpan;
use mpl::symbols::U8SliceTerminal;
use mpl::tree::AST;
use std::collections::HashMap;

mod chunk;
mod output;
mod sample;
mod variable;
mod wav;

/// ```
/// Wav = Riff ChunksAndData / f
/// ChunksAndData = Chunks Data / f
/// Chunks = Chunk Chunks / ()
///
/// Chunk = Fmt () / Chunk2
/// Chunk2 = Fact () / Chunk3
/// Chunk3 = Other () / f
///
/// // Riff Chunk
/// Riff = "RIFF" FileSize / f
/// FileSize = U32 Wave / f
/// Wave = "WAVE" () / f
///
/// // Fmt Chunk
/// Fmt = "Fmt " FmtSize / f
/// FmtSize = 16 FormatTag / FmtExt
/// FormatTag = U16 Channels / f
/// Channels = U16 SamplesPerSec / f
/// SamplesPerSec = U32 AvgBytesPerSec / f
/// AvgBytesPerSec = U32 BlockAlign / f
/// BlockAlign = U16 BitsPerSample / f
/// BitsPerSample = U16 () / f
///
/// FmtExt = 40 FormatTagWaveFormatExtensible / f
/// FormatTagWaveFormatExtensible = 0xFFFE WaveFormatExtensible / f
/// WaveFormatExtensible = Channels CbSize / f
/// CbSize = 22 ValidBitsPerSample / f
/// ValidBitsPerSample = U16 SamplesPerBlock / f
/// SamplesPerBlock = U16 Reserved / f
/// Reserved = U16 ChannelMask / f
/// ChannelMask = U32 SubFormat / f
/// SubFormat = U128 () / f
///
/// // Fact Chunk
/// Fact = "fact" FactSize / f
/// FactSize = 4 SampleLength / f
/// SampleLength = U32 () / f
///
/// // Other Chunk
/// Other = ???? OtherSize1 / f
/// OtherSize1 = 1 ? / OtherSize2
/// OtherSize2 = 2 ?? / OtherSize3
/// ...
/// OtherSize1023 = 1023 (omit) / OtherSize1024
/// OtherSize1024 = 1024 (omit) / f
///
/// // Data Chunk
/// Data = "data" DataSize / f
/// DataSize = U32 All / f
///
///
/// U16 = ?? () / f
/// U32 = ???? () / f
/// U128 = ???????? () / f
/// ```
fn main() {
    let mut rules = HashMap::new();

    rules.insert(
        WavVariable::Wav,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::Riff),
                RightRuleKind::V(WavVariable::ChunksAndData),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::ChunksAndData,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::Chunks),
                RightRuleKind::V(WavVariable::Data),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::Chunks,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::Chunk),
                RightRuleKind::V(WavVariable::Chunks),
            ),
            RightRuleKind::Epsilon,
        ),
    );

    rules.insert(
        WavVariable::Chunk,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::Fmt), RightRuleKind::Epsilon),
            RightRuleKind::V(WavVariable::Chunk2),
        ),
    );
    rules.insert(
        WavVariable::Chunk2,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::Fact), RightRuleKind::Epsilon),
            RightRuleKind::V(WavVariable::Chunk3),
        ),
    );
    rules.insert(
        WavVariable::Chunk3,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::Other), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );
    // Riff Chunk
    rules.insert(
        WavVariable::Riff,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Str("RIFF")),
                RightRuleKind::V(WavVariable::FileSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::FileSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U32),
                RightRuleKind::V(WavVariable::Wave),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::Wave,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Str("WAVE")),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::Failure,
        ),
    );

    // Fact Chunk
    rules.insert(
        WavVariable::Fact,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Str("fact")),
                RightRuleKind::V(WavVariable::FactSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::FactSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(4)),
                RightRuleKind::V(WavVariable::SampleLength),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::SampleLength,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::U32), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );

    // Fmt Chunk
    rules.insert(
        WavVariable::Fmt,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Str("fmt ")),
                RightRuleKind::V(WavVariable::FmtSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::FmtSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(16)),
                RightRuleKind::V(WavVariable::FormatTag),
            ),
            RightRuleKind::V(WavVariable::FmtExt),
        ),
    );
    rules.insert(
        WavVariable::FormatTag,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::Channels),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::Channels,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::SamplesPerSec),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::SamplesPerSec,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U32),
                RightRuleKind::V(WavVariable::AvgBytesPerSec),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::AvgBytesPerSec,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U32),
                RightRuleKind::V(WavVariable::BlockAlign),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::BlockAlign,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::BitsPerSample),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::BitsPerSample,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::U16), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );

    rules.insert(
        WavVariable::FmtExt,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(40)),
                RightRuleKind::V(WavVariable::FormatTagWaveFormatExtensible),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::FormatTagWaveFormatExtensible,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU16(0xFFFE)),
                RightRuleKind::V(WavVariable::WaveFormatExtensible),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::WaveFormatExtensible,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::Channels),
                RightRuleKind::V(WavVariable::CbSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::CbSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU16(22)),
                RightRuleKind::V(WavVariable::ValidBitsPerSample),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::ValidBitsPerSample,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::SamplesPerBlock),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::SamplesPerBlock,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::Reserved),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::Reserved,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U16),
                RightRuleKind::V(WavVariable::ChannelMask),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::ChannelMask,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavVariable::U32),
                RightRuleKind::V(WavVariable::SubFormat),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::SubFormat,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::U128), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );

    // Other Chunk
    rules.insert(
        WavVariable::Other,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::Any(4),
                RightRuleKind::V(WavVariable::OtherSize1),
            ),
            RightRuleKind::Failure,
        ),
    );

    rules.insert(
        WavVariable::OtherSize1,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(1)),
                RightRuleKind::Any(1),
            ),
            RightRuleKind::V(WavVariable::OtherSize24),
        ),
    );

    rules.insert(
        WavVariable::OtherSize24,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(24)),
                RightRuleKind::Any(24),
            ),
            RightRuleKind::V(WavVariable::OtherSize28),
        ),
    );

    rules.insert(
        WavVariable::OtherSize28,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(28)),
                RightRuleKind::Any(28),
            ),
            RightRuleKind::V(WavVariable::OtherSize602),
        ),
    );

    rules.insert(
        WavVariable::OtherSize602,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::LEU32(602)),
                RightRuleKind::Any(602),
            ),
            RightRuleKind::Failure,
        ),
    );

    // Data Chunk
    rules.insert(
        WavVariable::Data,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Str("data")),
                RightRuleKind::V(WavVariable::DataSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::DataSize,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(WavVariable::U32), RightRuleKind::All),
            RightRuleKind::Failure,
        ),
    );

    rules.insert(
        WavVariable::U16,
        RightRule::from_right_rule_kind(
            (RightRuleKind::Any(2), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::U32,
        RightRule::from_right_rule_kind(
            (RightRuleKind::Any(4), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavVariable::U128,
        RightRule::from_right_rule_kind(
            (RightRuleKind::Any(8), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );

    let input = include_bytes!("../base_drum.wav");
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, input.len() as u32);

    // let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, 12);
    // let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(12, 24);

    let result: Result<
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
    > = input.minimal_parse(&rules, &WavVariable::Wav, &all_of_the_span);

    let wav = result.unwrap().into_original().unwrap().to_wav();
    println!("{}", wav);
}
