use iced_x86::Instruction;

use crate::{StepResult, X86};

use super::helpers::*;

fn op1_mmm64(x86: &mut X86, instr: &iced_x86::Instruction) -> u64 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get64(instr.op1_register()),
        iced_x86::OpKind::Memory => read_u64(x86, x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

fn op1_mmm32(x86: &mut X86, instr: &iced_x86::Instruction) -> u32 {
    match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get64(instr.op1_register()) as u32,
        iced_x86::OpKind::Memory => x86.read_u32(x86_addr(x86, instr)),
        _ => unreachable!(),
    }
}

pub fn pxor_mm_mmm64(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_mmm64(x86, instr);
    rm64_x(x86, instr, |_x86, x| x ^ y);
    Ok(())
}

pub fn movd_mm_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr) as u64;
    rm64_x(x86, instr, |_x86, _x| y);
    Ok(())
}

pub fn punpcklbw_mm_mmm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_mmm32(x86, instr);
    rm64_x(x86, instr, |_x86, x| {
        let x = x as u32; // instr only uses low 32 bits of x
        (((y >> 24) & 0xFF) as u64) << 56
            | (((x >> 24) & 0xFF) as u64) << 48
            | (((y >> 16) & 0xFF) as u64) << 40
            | (((x >> 16) & 0xFF) as u64) << 32
            | (((y >> 8) & 0xFF) as u64) << 24
            | (((x >> 8) & 0xFF) as u64) << 16
            | (((y >> 0) & 0xFF) as u64) << 8
            | (((x >> 0) & 0xFF) as u64) << 0
    });
    Ok(())
}

pub fn pmullw_mm_mmm64(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_mmm64(x86, instr);
    rm64_x(x86, instr, |_x86, x| {
        let t0 = (((x >> 0) & 0xFFFF) as i16 as u32) * (((y >> 0) & 0xFFFF) as i16 as u32);
        let t1 = (((x >> 16) & 0xFFFF) as i16 as u32) * (((y >> 16) & 0xFFFF) as i16 as u32);
        let t2 = (((x >> 32) & 0xFFFF) as i16 as u32) * (((y >> 32) & 0xFFFF) as i16 as u32);
        let t3 = (((x >> 48) & 0xFFFF) as i16 as u32) * (((y >> 48) & 0xFFFF) as i16 as u32);
        (t3 as u64) << 48 | (t2 as u64) << 32 | (t1 as u64) << 16 | (t0 as u64)
    });
    Ok(())
}

pub fn psrlw_mm_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    rm64_x(x86, instr, |_x86, x| {
        (((x >> 0) & 0xFFFF) >> y) << 0
            | (((x >> 16) & 0xFFFF) >> y) << 16
            | (((x >> 32) & 0xFFFF) >> y) << 32
            | (((x >> 48) & 0xFFFF) >> y) << 48
    });
    Ok(())
}
