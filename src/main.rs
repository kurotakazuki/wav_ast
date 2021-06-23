use crate::output::WavOutput;
use crate::rules::WavRules;
use crate::variable::WavVariable;
use mpl::parse::Parse;
use mpl::span::StartAndLenSpan;
use mpl::tree::AST;

mod chunk;
mod output;
mod rules;
mod sample;
mod variable;
mod wav;

/// ```
/// Wav = Riff Chunks / f
/// Chunks = Chunk Chunks / ()
///
/// Chunk = Fmt () / Chunk2
/// Chunk2 = Data () / Other
///
/// // Riff Chunk
/// Riff = "RIFF" FileSize / f
/// FileSize = U32 "WAVE" / f
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
    let wav_rules = WavRules;

    let input = include_bytes!("../base_drum.wav");
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, input.len() as u32);

    let result: Result<
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
        AST<WavOutput, WavVariable, StartAndLenSpan<u32, u32>>,
    > = input.minimal_parse(&wav_rules, &WavVariable::Wav, &all_of_the_span);

    let wav = result.unwrap().into_original().unwrap().into_wav();
    println!("{}", wav);
    println!("{:#?}", wav);
}
