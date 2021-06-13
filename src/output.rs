use crate::chunk::{DataChunk, FactChunk, FmtChunk, OtherChunk, RiffChunk};
use crate::variable::WavVariable;
use crate::wav::Wav;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::tree::{AST, CST};
use std::convert::TryInto;

#[derive(Debug)]
pub enum WavOutput<'a> {
    Wav(Wav<'a>),
    Riff(RiffChunk),
    Fmt(FmtChunk),
    Fact(Option<FactChunk>),
    Other(OtherChunk<'a>),
    Data(DataChunk),

    U16(u16),
    U32(u32),
    U128(u128),
}

impl<'input> Output<'input, [u8], WavVariable, StartAndLenSpan<u32, u32>> for WavOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>,
    ) -> AST<Self, WavVariable, StartAndLenSpan<u32, u32>> {
        match cst.node.value {
            // WavVariable::Riff => {
            //     let ast = cst.node.equal;

            //     let riff = RiffChunk::new(size);
            //     AST::from_cst_and_output(cst, Some(WavOutput::Riff(riff)))
            // }
            // WavVariable::Wav => {

            //     let ast = cst.node.equal;

            //     AST::from_cst_and_output(cst, Some(WavOutput::U32(n)))
            // }
            WavVariable::U16 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u16::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_cst_and_output(cst.into_omit(), Some(WavOutput::U16(n)))
            }
            WavVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_cst_and_output(cst.into_omit(), Some(WavOutput::U32(n)))
            }
            WavVariable::U128 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u128::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_cst_and_output(cst.into_omit(), Some(WavOutput::U128(n)))
            }
            _ => AST::from_cst(cst),
        }
    }
}
