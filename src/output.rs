use crate::chunk::{DataChunk, FactChunk, FmtChunk, OtherChunk, RiffChunk};
use crate::variable::WavVariable;
use crate::wav::Wav;
use mpl::choice::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::tree::{ASTKind, AST, CST};
use std::convert::TryInto;

#[derive(Clone, Debug)]
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
            WavVariable::Riff => {
                let span = cst.span;
                let size: u32 = match cst.node.equal {
                    Choice::First(first) => match first.rhs.node {
                        ASTKind::InternalNode(equivalence) => match *equivalence.equal {
                            Choice::First(first) => match first.lhs.node {
                                ASTKind::LeafNode(TerminalSymbol::Original(WavOutput::U32(
                                    size,
                                ))) => size,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                let riff = RiffChunk::new(size);
                AST::from_leaf_node(TerminalSymbol::from_t(WavOutput::Riff(riff)), span)
            }
            // WavVariable::Wav => {

            //     let ast = cst.node.equal;

            //     AST::from_cst_and_output(cst, Some(WavOutput::U32(n)))
            // }

            // WavVariable::Fmt => {
            //     match &cst.node.equal {
            //         Choice::First(first) => match &first.rhs.node {
            //             ASTKind::InternalNode(equivalence) => {
            //                 match &*equivalence.equal {
            //                     // Normal Fmt
            //                     Choice::First(first) => {
            //                         dbg!(&first);
            //                         todo!()
            //                     },
            //                     // Fmt Extensible
            //                     Choice::Second(second) => {
            //                         todo!()
            //                     },
            //                 }
            //             }
            //             _ => unreachable!(),
            //         },
            //         _ => unreachable!(),
            //     }

            //     // let fmt = FmtChunk::new(size);
            //     // AST::from_cst_and_output(cst.into_omit(), Some(WavOutput::Fmt(fmt)))
            // }
            WavVariable::Channels => {
                dbg!(cst.node);
                todo!()
            }

            WavVariable::U16 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u16::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_t(WavOutput::U16(n)), cst.span)
            }
            WavVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_t(WavOutput::U32(n)), cst.span)
            }
            WavVariable::U128 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let n = u128::from_le_bytes(input[lo..hi].try_into().unwrap());
                AST::from_leaf_node(TerminalSymbol::from_t(WavOutput::U128(n)), cst.span)
            }
            _ => AST::from_cst(cst),
        }
    }
}
