use crate::chunk::{DataChunk, FactChunk, FmtChunk, OtherChunk, RiffChunk};
use crate::variable::WavVariable;
use crate::wav::Wav;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::tree::{AST, CST};
use std::convert::TryInto;

pub enum WavOutput<'a> {
    Wav(Wav<'a>),
    Riff(RiffChunk),
    Fmt(FmtChunk),
    Fact(Option<FactChunk>),
    Other(OtherChunk<'a>),
    Data(DataChunk),
}

impl<'input> Output<'input, [u8], WavVariable, StartAndLenSpan<u32, u32>> for WavOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>,
    ) -> AST<Self, WavVariable, StartAndLenSpan<u32, u32>> {
        let get_u16 = |cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>| -> u16 {
            let lo = cst.span.start as usize;
            let hi = cst.span.hi(input) as usize;
            u16::from_le_bytes(input[lo..hi].try_into().unwrap())
        };
        let get_u32 = |cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>| -> u32 {
            let lo = cst.span.start as usize;
            let hi = cst.span.hi(input) as usize;
            u32::from_le_bytes(input[lo..hi].try_into().unwrap())
        };
        let get_u128 = |cst: CST<Self, WavVariable, StartAndLenSpan<u32, u32>>| -> u128 {
            let lo = cst.span.start as usize;
            let hi = cst.span.hi(input) as usize;
            u128::from_le_bytes(input[lo..hi].try_into().unwrap())
        };
        match cst.node.value {
            // WavVariable::Wav => {

            //     let ast = cst.node.equal;

            //     AST::from_cst_and_output(cst, Some(WavOutput::U32(n)))
            // }
            _ => AST::from_cst(cst),
        }
    }
}
