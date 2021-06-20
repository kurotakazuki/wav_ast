use crate::output::WavOutput;
use crate::variable::WavVariable;
use mpl::parse::Parse;
use mpl::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpl::span::StartAndLenSpan;
use mpl::symbols::U8SliceTerminal;
use mpl::tree::AST;

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
    let set_rule = |variable, first_lhs, first_rhs, second| -> Rule<U8SliceTerminal, WavVariable> {
        Rule::new(
            variable,
            RightRule::from_right_rule_kind((first_lhs, first_rhs), second),
        )
    };

    let wav_rule = set_rule(
        WavVariable::Wav,
        RightRuleKind::V(WavVariable::Riff),
        RightRuleKind::V(WavVariable::ChunksAndData),
        RightRuleKind::Failure,
    );
    let chunks_and_data_rule = set_rule(
        WavVariable::ChunksAndData,
        RightRuleKind::V(WavVariable::Chunks),
        RightRuleKind::V(WavVariable::Data),
        RightRuleKind::Failure,
    );
    let chunks_rule = set_rule(
        WavVariable::Chunks,
        RightRuleKind::V(WavVariable::Chunk),
        RightRuleKind::V(WavVariable::Chunks),
        RightRuleKind::Epsilon,
    );

    let chunk_rule = set_rule(
        WavVariable::Chunk,
        RightRuleKind::V(WavVariable::Fmt),
        RightRuleKind::Epsilon,
        RightRuleKind::V(WavVariable::Chunk2),
    );
    let chunk2_rule = set_rule(
        WavVariable::Chunk2,
        RightRuleKind::V(WavVariable::Fact),
        RightRuleKind::Epsilon,
        RightRuleKind::V(WavVariable::Chunk3),
    );
    let chunk3_rule = set_rule(
        WavVariable::Chunk3,
        RightRuleKind::V(WavVariable::Other),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    // Riff Chunk
    let riff_rule = set_rule(
        WavVariable::Riff,
        RightRuleKind::T(U8SliceTerminal::Str("RIFF")),
        RightRuleKind::V(WavVariable::FileSize),
        RightRuleKind::Failure,
    );
    let file_size_rule = set_rule(
        WavVariable::FileSize,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::V(WavVariable::Wave),
        RightRuleKind::Failure,
    );
    let wave_rule = set_rule(
        WavVariable::Wave,
        RightRuleKind::T(U8SliceTerminal::Str("WAVE")),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    // Fmt Chunk
    let fmt_rule = set_rule(
        WavVariable::Fmt,
        RightRuleKind::T(U8SliceTerminal::Str("fmt ")),
        RightRuleKind::V(WavVariable::FmtSize),
        RightRuleKind::Failure,
    );
    let fmt_size_rule = set_rule(
        WavVariable::FmtSize,
        RightRuleKind::T(U8SliceTerminal::LEU32(16)),
        RightRuleKind::V(WavVariable::FormatTag),
        RightRuleKind::V(WavVariable::FmtExt),
    );
    let format_tag_rule = set_rule(
        WavVariable::FormatTag,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::Channels),
        RightRuleKind::Failure,
    );
    let channels_rule = set_rule(
        WavVariable::Channels,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::SamplesPerSec),
        RightRuleKind::Failure,
    );
    let samples_per_sec_rule = set_rule(
        WavVariable::SamplesPerSec,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::V(WavVariable::AvgBytesPerSec),
        RightRuleKind::Failure,
    );
    let avg_bytes_per_sec_rule = set_rule(
        WavVariable::AvgBytesPerSec,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::V(WavVariable::BlockAlign),
        RightRuleKind::Failure,
    );
    let block_align_rule = set_rule(
        WavVariable::BlockAlign,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::BitsPerSample),
        RightRuleKind::Failure,
    );
    let bits_per_sample_rule = set_rule(
        WavVariable::BitsPerSample,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    let fmt_ext_rule = set_rule(
        WavVariable::FmtExt,
        RightRuleKind::T(U8SliceTerminal::LEU32(40)),
        RightRuleKind::V(WavVariable::FormatTagWaveFormatExtensible),
        RightRuleKind::Failure,
    );
    let format_tag_wave_format_extensible_rule = set_rule(
        WavVariable::FormatTagWaveFormatExtensible,
        RightRuleKind::T(U8SliceTerminal::LEU16(0xFFFE)),
        RightRuleKind::V(WavVariable::WaveFormatExtensible),
        RightRuleKind::Failure,
    );
    let wave_format_extensible_rule = set_rule(
        WavVariable::WaveFormatExtensible,
        RightRuleKind::V(WavVariable::Channels),
        RightRuleKind::V(WavVariable::CbSize),
        RightRuleKind::Failure,
    );
    let cb_size_rule = set_rule(
        WavVariable::CbSize,
        RightRuleKind::T(U8SliceTerminal::LEU16(22)),
        RightRuleKind::V(WavVariable::ValidBitsPerSample),
        RightRuleKind::Failure,
    );
    let valid_bits_per_sample_rule = set_rule(
        WavVariable::ValidBitsPerSample,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::SamplesPerBlock),
        RightRuleKind::Failure,
    );
    let samples_per_block_rule = set_rule(
        WavVariable::SamplesPerBlock,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::Reserved),
        RightRuleKind::Failure,
    );
    let reserved_rule = set_rule(
        WavVariable::Reserved,
        RightRuleKind::V(WavVariable::U16),
        RightRuleKind::V(WavVariable::ChannelMask),
        RightRuleKind::Failure,
    );
    let channel_mask_rule = set_rule(
        WavVariable::ChannelMask,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::V(WavVariable::SubFormat),
        RightRuleKind::Failure,
    );
    let sub_format_rule = set_rule(
        WavVariable::SubFormat,
        RightRuleKind::V(WavVariable::U128),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    // Fact Chunk
    let fact_rule = set_rule(
        WavVariable::Fact,
        RightRuleKind::T(U8SliceTerminal::Str("fact")),
        RightRuleKind::V(WavVariable::FactSize),
        RightRuleKind::Failure,
    );
    let fact_size_rule = set_rule(
        WavVariable::FactSize,
        RightRuleKind::T(U8SliceTerminal::LEU32(4)),
        RightRuleKind::V(WavVariable::SampleLength),
        RightRuleKind::Failure,
    );
    let sample_length_rule = set_rule(
        WavVariable::SampleLength,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    // Other Chunk
    let other_rule = set_rule(
        WavVariable::Other,
        RightRuleKind::Any(4),
        RightRuleKind::V(WavVariable::OtherSize1),
        RightRuleKind::Failure,
    );

    // Data Chunk
    let data_rule = set_rule(
        WavVariable::Data,
        RightRuleKind::T(U8SliceTerminal::Str("data")),
        RightRuleKind::V(WavVariable::DataSize),
        RightRuleKind::Failure,
    );
    let data_size_rule = set_rule(
        WavVariable::DataSize,
        RightRuleKind::V(WavVariable::U32),
        RightRuleKind::All,
        RightRuleKind::Failure,
    );

    let u16_rule = set_rule(
        WavVariable::U16,
        RightRuleKind::Any(2),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );
    let u32_rule = set_rule(
        WavVariable::U32,
        RightRuleKind::Any(4),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );
    let u128_rule = set_rule(
        WavVariable::U128,
        RightRuleKind::Any(8),
        RightRuleKind::Epsilon,
        RightRuleKind::Failure,
    );

    let mut rules = Rules::new();
    rules.insert_rule(wav_rule);
    rules.insert_rule(chunks_and_data_rule);
    rules.insert_rule(chunks_rule);

    rules.insert_rule(chunk_rule);
    rules.insert_rule(chunk2_rule);
    rules.insert_rule(chunk3_rule);
    // Riff Chunk
    rules.insert_rule(riff_rule);
    rules.insert_rule(file_size_rule);
    rules.insert_rule(wave_rule);

    // Fact Chunk
    rules.insert_rule(fact_rule);
    rules.insert_rule(fact_size_rule);
    rules.insert_rule(sample_length_rule);

    // Fmt Chunk
    rules.insert_rule(fmt_rule);
    rules.insert_rule(fmt_size_rule);
    rules.insert_rule(format_tag_rule);
    rules.insert_rule(channels_rule);
    rules.insert_rule(samples_per_sec_rule);
    rules.insert_rule(avg_bytes_per_sec_rule);
    rules.insert_rule(block_align_rule);
    rules.insert_rule(bits_per_sample_rule);

    rules.insert_rule(fmt_ext_rule);
    rules.insert_rule(format_tag_wave_format_extensible_rule);
    rules.insert_rule(wave_format_extensible_rule);
    rules.insert_rule(cb_size_rule);
    rules.insert_rule(valid_bits_per_sample_rule);
    rules.insert_rule(samples_per_block_rule);
    rules.insert_rule(reserved_rule);
    rules.insert_rule(channel_mask_rule);
    rules.insert_rule(sub_format_rule);

    // Other Chunk
    rules.insert_rule(other_rule);

    rules.insert_rule(set_rule(
        WavVariable::OtherSize1,
        RightRuleKind::T(U8SliceTerminal::LEU32(1)),
        RightRuleKind::Any(1),
        RightRuleKind::V(WavVariable::OtherSize24),
    ));

    rules.insert_rule(set_rule(
        WavVariable::OtherSize24,
        RightRuleKind::T(U8SliceTerminal::LEU32(24)),
        RightRuleKind::Any(24),
        RightRuleKind::V(WavVariable::OtherSize28),
    ));

    rules.insert_rule(set_rule(
        WavVariable::OtherSize28,
        RightRuleKind::T(U8SliceTerminal::LEU32(28)),
        RightRuleKind::Any(28),
        RightRuleKind::V(WavVariable::OtherSize602),
    ));

    rules.insert_rule(set_rule(
        WavVariable::OtherSize602,
        RightRuleKind::T(U8SliceTerminal::LEU32(602)),
        RightRuleKind::Any(602),
        RightRuleKind::Failure,
    ));

    // Data Chunk
    rules.insert_rule(data_rule);
    rules.insert_rule(data_size_rule);

    rules.insert_rule(u16_rule);
    rules.insert_rule(u32_rule);
    rules.insert_rule(u128_rule);

    let input = include_bytes!("../base_drum.wav");
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, input.len() as u32);

    // let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, 12);
    // let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(12, 24);

    let result: Result<
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
    > = input.minimal_parse(&rules, &WavVariable::Wav, &all_of_the_span);

    println!("{:#?}", result);
}
