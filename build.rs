use std::{
    fs::File,
    io,
    io::{Read, Write},
};

fn main() -> io::Result<()> {
    // // Files
    // let mut other_size_in_enum = File::create("other_size_in_enum.rs")?;
    // for i in 1..=1024 {
    //     writeln!(&mut other_size_in_enum, "OtherSize{},", i)?;
    // }
    let mut other_size_rules = File::create("other_size_rules.rs")?;

    for i in 1..1024 {
        writeln!(&mut other_size_rules, "    rules.insert_rule(set_rule(")?;
        writeln!(
            &mut other_size_rules,
            "            WavVariable::OtherSize{},",
            i
        )?;
        writeln!(
            &mut other_size_rules,
            "            RightRuleKind::T(U8SliceTerminal::LEU32({})),",
            i
        )?;
        writeln!(
            &mut other_size_rules,
            "            RightRuleKind::Any({}),",
            i
        )?;
        writeln!(
            &mut other_size_rules,
            "            RightRuleKind::V(WavVariable::OtherSize{}),",
            i + 1
        )?;
        writeln!(&mut other_size_rules, "    ));")?;
    }
    // 1024
    let i = 1024;
    writeln!(&mut other_size_rules, "    rules.insert_rule(set_rule(")?;
    writeln!(
        &mut other_size_rules,
        "            WavVariable::OtherSize{},",
        i
    )?;
    writeln!(
        &mut other_size_rules,
        "            RightRuleKind::T(U8SliceTerminal::LEU32({})),",
        i
    )?;
    writeln!(
        &mut other_size_rules,
        "            RightRuleKind::Any({}),",
        i
    )?;
    writeln!(&mut other_size_rules, "            RightRuleKind::Failure,")?;
    writeln!(&mut other_size_rules, "    ));")?;

    Ok(())
}
