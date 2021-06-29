use crate::chunk::{DataChunk, FmtChunk, FormatTag, OtherChunk, RiffChunk, WaveFormatExtensible};
use crate::sample::Sample;
use crate::variable::WavVariable;
use crate::wav::Wav;
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{Node, AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub enum WavOutput<'a> {
    Wav(Wav<'a>),

    Riff(RiffChunk),
    Fmt(FmtChunk),
    Other(OtherChunk<'a>),
    // Data(DataChunk),
    U16(u16),
    U32(u32),
    U128(u128),
}

impl<'a> WavOutput<'a> {
    pub fn into_wav(self) -> Wav<'a> {
        match self {
            Self::Wav(wav) => wav,
            _ => panic!(),
        }
    }

    pub fn into_riff(self) -> RiffChunk {
        match self {
            Self::Riff(riff) => riff,
            _ => panic!(),
        }
    }

    pub fn into_fmt(self) -> FmtChunk {
        match self {
            Self::Fmt(fmt) => fmt,
            _ => panic!(),
        }
    }

    pub fn into_u16(self) -> u16 {
        match self {
            Self::U16(n) => n,
            _ => panic!(),
        }
    }
    pub fn into_u32(self) -> u32 {
        match self {
            Self::U32(n) => n,
            _ => panic!(),
        }
    }
    pub fn into_u128(self) -> u128 {
        match self {
            Self::U128(n) => n,
            _ => panic!(),
        }
    }
}

impl<'input> Output<'input, [u8], WavVariable, StartAndLenSpan<u32, u32>> for WavOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<WavVariable, StartAndLenSpan<u32, u32>, Self>,
    ) -> AST<WavVariable, StartAndLenSpan<u32, u32>, Self> {
        match cst.node.value {
            WavVariable::Wav => {
                let span = cst.span;
                let wav_v = cst.node.equal.into_first().unwrap();
                let riff = wav_v.lhs.into_original().unwrap().into_riff();

                let mut fmt = None;
                let mut others = Vec::new();

                // Warning: This will panic if there is no chunk.
                let mut chunks_v = wav_v.rhs.into_first().unwrap();

                loop {
                    match chunks_v.lhs.node {
                        Node::Leaf(n) => match n.into_original().unwrap() {
                            WavOutput::Fmt(c) => fmt = Some(c),
                            WavOutput::Other(c) => others.push(c),
                            _ => unreachable!(),
                        },
                        Node::Internal(n) => {
                            // Data Chunk
                            let data_v = n.into_first().unwrap();

                            let data_size_v = data_v.rhs.into_first().unwrap();
                            let data_size: u32 =
                                data_size_v.lhs.into_original().unwrap().into_u32();

                            let data_span = data_size_v.rhs.span;

                            if data_span.len != data_size {
                                panic!("data size and data span is not equal");
                            }

                            if let Some(fmt) = fmt {
                                let sample_frames = fmt.sample_frames(data_size) as usize;
                                let channels = fmt.channels as usize;
                                let mut data_vec: Vec<Vec<Sample>> =
                                    vec![Vec::with_capacity(channels); sample_frames];

                                let data_span_lo = data_span.start as usize;
                                for (sample_index, sample_frame_vec) in
                                    data_vec.iter_mut().enumerate().take(sample_frames)
                                {
                                    for channel_index in 0..channels {
                                        let relative_pos = sample_index * fmt.block_align as usize
                                            + channel_index * (fmt.bytes_per_sample() as usize);
                                        match fmt.bits_per_sample {
                                            8 => {
                                                let sample = u8::from_le_bytes(
                                                    input[data_span_lo + relative_pos..][..1]
                                                        .try_into()
                                                        .unwrap(),
                                                );
                                                sample_frame_vec.push(sample.into());
                                            }
                                            16 => {
                                                let sample = u16::from_le_bytes(
                                                    input[data_span_lo + relative_pos..][..2]
                                                        .try_into()
                                                        .unwrap(),
                                                );
                                                sample_frame_vec.push(sample.into());
                                            }
                                            32 => match fmt.format_tag {
                                                FormatTag::IEEEFloatingPoint => {
                                                    let sample = f32::from_le_bytes(
                                                        input[data_span_lo + relative_pos..][..4]
                                                            .try_into()
                                                            .unwrap(),
                                                    );
                                                    sample_frame_vec.push(sample.into());
                                                }
                                                _ => {
                                                    unimplemented!()
                                                }
                                            },
                                            64 => match fmt.format_tag {
                                                FormatTag::IEEEFloatingPoint => {
                                                    let sample = f64::from_le_bytes(
                                                        input[data_span_lo + relative_pos..][..8]
                                                            .try_into()
                                                            .unwrap(),
                                                    );
                                                    sample_frame_vec.push(sample.into());
                                                }
                                                _ => {
                                                    unimplemented!()
                                                }
                                            },
                                            _ => {
                                                unimplemented!()
                                            }
                                        }
                                    }
                                }

                                let data = DataChunk::new(data_size, data_vec);

                                let wav = Wav::new(riff, fmt, others, data);

                                return AST::from_leaf(
                                    TerminalSymbol::from_original(WavOutput::Wav(wav)),
                                    span,
                                );
                            } else {
                                panic!("No Fmt Chunk");
                            }
                        }
                    }

                    if let Some(first) = chunks_v.rhs.into_first() {
                        chunks_v = first;
                    } else {
                        // When Epsilon
                        panic!("No Data Chunk")
                    }
                }
            }

            WavVariable::Chunk => {
                match cst.node.equal {
                    // Fmt Chunk
                    Choice::First(first) => first.lhs,
                    // Data or Other Chunk
                    Choice::Second(second) => match *second.0.into_internal().unwrap().equal {
                        // Data Chunk
                        Choice::First(first) => first.lhs,
                        // Other Chunk
                        Choice::Second(second) => second.0,
                    },
                }
            }

            WavVariable::Riff => {
                let span = cst.span;
                let size: u32 = match cst
                    .node
                    .equal
                    .into_first()
                    .unwrap()
                    .rhs
                    .into_first()
                    .unwrap()
                    .lhs
                    .into_original()
                    .unwrap()
                {
                    WavOutput::U32(size) => size,
                    _ => unreachable!(),
                };

                let riff = RiffChunk::new(size);
                AST::from_leaf(TerminalSymbol::from_original(WavOutput::Riff(riff)), span)
            }

            WavVariable::Fmt => {
                let span = cst.span;

                match *cst
                    .node
                    .equal
                    .into_first()
                    .unwrap()
                    .rhs
                    .into_internal()
                    .unwrap()
                    .equal
                {
                    // Normal Fmt
                    Choice::First(first) => {
                        let size: u32 = 16;

                        let format_tag_v = first.rhs.into_first().unwrap();
                        let format_tag: FormatTag =
                            format_tag_v.lhs.into_original().unwrap().into_u16().into();

                        let mut fmt = format_tag_v.rhs.into_original().unwrap().into_fmt();

                        fmt.chunk_header.size = size;
                        fmt.format_tag = format_tag;

                        AST::from_leaf(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
                    }
                    // Fmt Extensible
                    Choice::Second(second) => {
                        let size: u32 = 40;
                        let format_tag: FormatTag = FormatTag::WaveFormatExtensible;
                        let cb_size: Option<u16> = Some(22);

                        let format_tag_wave_format_extensible_v = second.0.into_first().unwrap();

                        let wave_format_extensible_v = format_tag_wave_format_extensible_v
                            .rhs
                            .into_first()
                            .unwrap();

                        let mut fmt = wave_format_extensible_v
                            .lhs
                            .into_original()
                            .unwrap()
                            .into_fmt();

                        let cb_size_v = wave_format_extensible_v.rhs.into_first().unwrap();

                        let valid_bits_per_sample_v = cb_size_v.rhs.into_first().unwrap();
                        let valid_bits_per_sample: u16 = valid_bits_per_sample_v
                            .lhs
                            .into_original()
                            .unwrap()
                            .into_u16();

                        let samples_per_block_v = valid_bits_per_sample_v.rhs.into_first().unwrap();
                        let samples_per_block: u16 =
                            samples_per_block_v.lhs.into_original().unwrap().into_u16();

                        let reserved_v = samples_per_block_v.rhs.into_first().unwrap();
                        let reserved: u16 = reserved_v.lhs.into_original().unwrap().into_u16();

                        let channel_mask_v = reserved_v.rhs.into_first().unwrap();
                        let channel_mask: u32 =
                            channel_mask_v.lhs.into_original().unwrap().into_u32();

                        let sub_format: u128 =
                            channel_mask_v.rhs.into_original().unwrap().into_u128();

                        fmt.chunk_header.size = size;
                        fmt.format_tag = format_tag;
                        fmt.cb_size = cb_size;

                        fmt.wave_format_extensible = Some(WaveFormatExtensible::new(
                            valid_bits_per_sample,
                            samples_per_block,
                            reserved,
                            channel_mask,
                            sub_format,
                        ));

                        AST::from_leaf(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
                    }
                }
            }
            WavVariable::Channels => {
                let span = cst.span;

                let channels_v = cst.node.equal.into_first().unwrap();
                let channels: u16 = channels_v.lhs.into_original().unwrap().into_u16();

                let samples_per_sec_v = channels_v.rhs.into_first().unwrap();
                let samples_per_sec: u32 =
                    samples_per_sec_v.lhs.into_original().unwrap().into_u32();

                let avg_bytes_per_sec_v = samples_per_sec_v.rhs.into_first().unwrap();
                let avg_bytes_per_sec: u32 =
                    avg_bytes_per_sec_v.lhs.into_original().unwrap().into_u32();

                let block_align_v = avg_bytes_per_sec_v.rhs.into_first().unwrap();

                let block_align: u16 = block_align_v.lhs.into_original().unwrap().into_u16();

                let bits_per_sample: u16 = block_align_v
                    .rhs
                    .into_first()
                    .unwrap()
                    .lhs
                    .into_original()
                    .unwrap()
                    .into_u16();

                // Add some unknown values
                let size = 16;
                let format_tag = FormatTag::from(0xFFFF);
                let cb_size = None;
                let wave_format_extensible = None;

                let fmt = FmtChunk::new(
                    size,
                    format_tag,
                    channels,
                    samples_per_sec,
                    avg_bytes_per_sec,
                    block_align,
                    bits_per_sample,
                    cb_size,
                    wave_format_extensible,
                );
                AST::from_leaf(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
            }

            WavVariable::Other => {
                let span = cst.span;
                let lo = span.start as usize;
                let hi = span.hi(input) as usize;

                let size = u32::from_le_bytes(input[lo + 4..lo + 8].try_into().unwrap());

                let four_cc = input[lo..lo + 4].try_into().unwrap();

                let other = OtherChunk::new(four_cc, size, &input[lo + 8..hi]);

                AST::from_leaf(TerminalSymbol::from_original(WavOutput::Other(other)), span)
            }

            WavVariable::U16 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u16::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf(TerminalSymbol::from_original(WavOutput::U16(n)), cst.span)
            }
            WavVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf(TerminalSymbol::from_original(WavOutput::U32(n)), cst.span)
            }
            WavVariable::U128 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u128::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf(TerminalSymbol::from_original(WavOutput::U128(n)), cst.span)
            }
            _ => AST::from_cst(cst),
        }
    }
}
