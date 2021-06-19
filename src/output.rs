use crate::chunk::{DataChunk, FactChunk, FmtChunk, FormatTag, OtherChunk, RiffChunk, WaveFormatExtensible};
use crate::variable::WavVariable;
use crate::wav::Wav;
use mpl::choice::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::tree::{AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub enum WavOutput<'a> {
    Wav(Wav<'a>),
    Riff(RiffChunk),
    Fmt(FmtChunk),
    Fact(FactChunk),
    Other(OtherChunk<'a>),
    Data(DataChunk),

    U16(u16),
    U32(u32),
    U128(u128),
}

impl WavOutput<'_> {
    fn to_fmt(self) -> FmtChunk {
        match self {
            Self::Fmt(fmt) => fmt,
            _ => panic!(),
        }
    }

    fn to_u16(self) -> u16 {
        match self {
            Self::U16(n) => n,
            _ => panic!(),
        }
    }
    fn to_u32(self) -> u32 {
        match self {
            Self::U32(n) => n,
            _ => panic!(),
        }
    }
    fn to_u128(self) -> u128 {
        match self {
            Self::U128(n) => n,
            _ => panic!(),
        }
    }
}

impl<'input> Output<'input, [u8], WavVariable, StartAndLenSpan<u32, u32>> for WavOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>,
    ) -> AST<Self, WavVariable, StartAndLenSpan<u32, u32>> {
        match cst.node.value {
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
                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::Riff(riff)), span)
            }
            // WavVariable::Wav => {

            //     let ast = cst.node.equal;

            //     AST::from_cst_and_output(cst, Some(WavOutput::U32(n)))
            // }

            WavVariable::Fact => {
                let span = cst.span;
                let fact_size = 4;

                let fact_v = cst.node.equal.into_first().unwrap();

                let fact_size_v = fact_v.rhs.into_first().unwrap();

                let sample_length_v = fact_size_v.rhs.into_first().unwrap();
                let sample_length: u32 = sample_length_v.lhs.into_original().unwrap().to_u32();

                let fact = FactChunk::new(fact_size, sample_length);

                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::Fact(fact)), span)
            }

            WavVariable::Fmt => {
                let span = cst.span;

                match *cst.node.equal.into_first().unwrap().rhs.into_internal_node().unwrap().equal {
                    // Normal Fmt
                    Choice::First(first) => {
                        let size: u32 = 16;

                        let format_tag_v = first.rhs.into_first().unwrap();
                        let format_tag: FormatTag = format_tag_v.lhs.into_original().unwrap().to_u16().into();

                        let mut fmt = format_tag_v.rhs.into_original().unwrap().to_fmt();

                        fmt.chunk_header.size = size;
                        fmt.format_tag = format_tag;
                        
                        AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
                    },
                    // Fmt Extensible
                    Choice::Second(second) => {
                        let size: u32 = 40;
                        let format_tag: FormatTag = FormatTag::WaveFormatExtensible;
                        let cb_size: Option<u16> = Some(22);
                        
                        let format_tag_wave_format_extensible_v = second.0.into_first().unwrap();
                        
                        let wave_format_extensible_v = format_tag_wave_format_extensible_v.rhs.into_first().unwrap();

                        let mut fmt = wave_format_extensible_v.lhs.into_original().unwrap().to_fmt();

                        let cb_size_v = wave_format_extensible_v.rhs.into_first().unwrap();

                        let valid_bits_per_sample_v = cb_size_v.rhs.into_first().unwrap();
                        let valid_bits_per_sample: u16 = valid_bits_per_sample_v.lhs.into_original().unwrap().to_u16();
                        
                        let samples_per_block_v = valid_bits_per_sample_v.rhs.into_first().unwrap();
                        let samples_per_block: u16 = samples_per_block_v.lhs.into_original().unwrap().to_u16();

                        let reserved_v = samples_per_block_v.rhs.into_first().unwrap();
                        let reserved: u16 = reserved_v.lhs.into_original().unwrap().to_u16();

                        let channel_mask_v = reserved_v.rhs.into_first().unwrap();
                        let channel_mask: u32 = channel_mask_v.lhs.into_original().unwrap().to_u32();

                        let sub_format: u128 = channel_mask_v.rhs.into_original().unwrap().to_u128();

                        fmt.chunk_header.size = size;
                        fmt.format_tag = format_tag;
                        fmt.cb_size = cb_size;

                        fmt.wave_format_extensible = Some(WaveFormatExtensible::new(
                            valid_bits_per_sample,
                            samples_per_block,
                            reserved,
                            channel_mask,
                            sub_format
                        ));

                        AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
                    },
                }

            }
            WavVariable::Channels => {
                let span = cst.span;

                let channels_v = cst.node.equal.into_first().unwrap();
                let channels: u16 = channels_v.lhs.into_original().unwrap().to_u16();

                let samples_per_sec_v = channels_v.rhs.into_first().unwrap();
                let samples_per_sec: u32 = samples_per_sec_v.lhs.into_original().unwrap().to_u32();

                let avg_bytes_per_sec_v = samples_per_sec_v.rhs.into_first().unwrap();
                let avg_bytes_per_sec: u32 =
                    avg_bytes_per_sec_v.lhs.into_original().unwrap().to_u32();

                let block_align_v = avg_bytes_per_sec_v.rhs.into_first().unwrap();

                let block_align: u16 = block_align_v.lhs.into_original().unwrap().to_u16();

                let bits_per_sample: u16 = block_align_v.rhs.into_first().unwrap().lhs.into_original().unwrap().to_u16();

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
                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::Fmt(fmt)), span)
            }

            WavVariable::U16 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u16::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::U16(n)), cst.span)
            }
            WavVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::U32(n)), cst.span)
            }
            WavVariable::U128 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u128::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_original(WavOutput::U128(n)), cst.span)
            }
            _ => AST::from_cst(cst),
        }
    }
}
