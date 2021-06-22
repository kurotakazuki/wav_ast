// use std::{fs::File, io, io::Write};

// fn main() -> io::Result<()> {
fn main() {
    // // Files
    // // let mut other_size_in_enum = File::create("other_size_in_enum.rs")?;
    // // for i in 1..=1024 {
    // //     writeln!(&mut other_size_in_enum, "OtherSize{},", i)?;
    // // }
    // let mut other_size_right_rules_enum = File::create("other_size_right_rules_enum.rs")?;
    // for i in 1..=1024 {
    //     writeln!(&mut other_size_right_rules_enum, "            OtherSize{} => &Self::OTHER_SIZE{}_RIGHT_RULE,", i, i)?;
    // }

    // let mut other_size_right_rules = File::create("other_size_right_rules.rs")?;

    // for i in 1..1024 {
    //     writeln!(&mut other_size_right_rules, "const OTHER_SIZE{}_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {{", i)?;
    //     writeln!(
    //         &mut other_size_right_rules,
    //         "    first: First {{ lhs: E::T(TerminalSymbol::Original(LEU32({}))), rhs: E::T(TerminalSymbol::Metasymbol(Any({}))) }},",
    //         i,
    //         i
    //     )?;
    //     writeln!(
    //         &mut other_size_right_rules,
    //         "    second: Second(E::V(OtherSize{})),",
    //         i + 1
    //     )?;
    //     writeln!(&mut other_size_right_rules, "}};")?;
    // }
    // // 1024
    // let i = 1024;
    // writeln!(
    //     &mut other_size_right_rules,
    //     "const OTHER_SIZE{}_RIGHT_RULE: RightRule<U8SliceTerminal<'a>, WavVariable> = RightRule {{",
    //     i
    // )?;
    // writeln!(
    //     &mut other_size_right_rules,
    //     "    first: First {{ lhs: E::T(TerminalSymbol::Original(LEU32({}))), rhs: E::T(TerminalSymbol::Metasymbol(Any({}))) }},",
    //     i,
    //     i
    // )?;
    // writeln!(
    //     &mut other_size_right_rules,
    //     "    second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),"
    // )?;
    // writeln!(&mut other_size_right_rules, "}};")?;

    // Ok(())
}
